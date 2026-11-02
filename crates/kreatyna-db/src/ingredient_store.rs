//! Transactional SQLite implementation of the ingredient capability.
use std::collections::HashMap;

use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use kreatyna::ingredients::{
    Ingredient, IngredientId, IngredientWrite, IngredientWriteData,
    capabilities::{
        IngredientDeleteError, IngredientLoadError, IngredientSaveError, IngredientStoreMut,
    },
    name_key,
};

use super::{
    ingredient_models::{
        IngredientRow, LocalIngredientId, ModelError, NewIngredientRow, RemoteIngredientId, UnitRow,
    },
    schema::{ingredient_units, ingredients},
    sqlite::SqliteDatabase,
};

#[derive(Debug, thiserror::Error)]
enum StoreFailure {
    #[error(transparent)]
    Database(#[from] diesel::result::Error),
    #[error(transparent)]
    Model(#[from] ModelError),
    #[error("ingredient name already exists")]
    DuplicateName,
    #[error("ingredient was not found")]
    NotFound,
    #[error("stored ingredient or unit name key is inconsistent")]
    InvalidNameKey,
    #[error("stored reference unit has no ingredient")]
    OrphanUnit,
}

fn checked_unit(
    unit: UnitRow,
) -> Result<(i64, kreatyna::ingredients::ReferenceUnit), StoreFailure> {
    if unit.name_key != name_key(&unit.name) {
        return Err(StoreFailure::InvalidNameKey);
    }
    let parent = unit.ingredient_local_id.0;
    let (_, domain) = unit.into_domain()?;
    Ok((parent, domain))
}

fn checked_ingredient(
    parent: IngredientRow,
    units: Vec<kreatyna::ingredients::ReferenceUnit>,
) -> Result<Ingredient, StoreFailure> {
    if parent.name_key != name_key(&parent.name) {
        return Err(StoreFailure::InvalidNameKey);
    }
    Ok(parent.into_domain(units))
}

fn records(connection: &mut SqliteConnection) -> Result<Vec<Ingredient>, StoreFailure> {
    let parents = ingredients::table
        .select(IngredientRow::as_select())
        .load::<IngredientRow>(connection)?;
    let units = ingredient_units::table
        .order((
            ingredient_units::ingredient_local_id,
            ingredient_units::position,
        ))
        .select(UnitRow::as_select())
        .load::<UnitRow>(connection)?;
    let mut by_parent: HashMap<i64, Vec<_>> = HashMap::new();
    for unit in units {
        let (parent, domain) = checked_unit(unit)?;
        by_parent.entry(parent).or_default().push(domain);
    }
    let result = parents
        .into_iter()
        .map(|parent| {
            let units = by_parent.remove(&parent.local_id.0).unwrap_or_default();
            checked_ingredient(parent, units)
        })
        .collect::<Result<Vec<_>, _>>()?;
    if !by_parent.is_empty() {
        return Err(StoreFailure::OrphanUnit);
    }
    Ok(result)
}

fn record(
    connection: &mut SqliteConnection,
    local_id: LocalIngredientId,
) -> Result<Ingredient, StoreFailure> {
    let parent = ingredients::table
        .find(local_id)
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(connection)?;
    let units = ingredient_units::table
        .filter(ingredient_units::ingredient_local_id.eq(local_id))
        .order(ingredient_units::position)
        .select(UnitRow::as_select())
        .load::<UnitRow>(connection)?
        .into_iter()
        .map(|unit| checked_unit(unit).map(|(_, domain)| domain))
        .collect::<Result<Vec<_>, _>>()?;
    checked_ingredient(parent, units)
}

fn save_in_transaction(
    connection: &mut SqliteConnection,
    write: IngredientWrite,
) -> Result<Ingredient, StoreFailure> {
    let (id, data, existing) = match write {
        IngredientWrite::Create { data } => (IngredientId::generate(), data, None),
        IngredientWrite::Update { id, data } => {
            let local_id = ingredients::table
                .filter(ingredients::remote_id.eq(RemoteIngredientId::from(id)))
                .select(ingredients::local_id)
                .first::<LocalIngredientId>(connection)
                .optional()?
                .ok_or(StoreFailure::NotFound)?;
            (id, data, Some(local_id))
        }
    };
    let conflict = ingredients::table
        .filter(ingredients::name_key.eq(name_key(data.name())))
        .select(ingredients::local_id)
        .first::<LocalIngredientId>(connection)
        .optional()?;
    if conflict.is_some() && conflict != existing {
        return Err(StoreFailure::DuplicateName);
    }
    let row = NewIngredientRow::from_write(id, &data);
    let local_id = if let Some(local_id) = existing {
        diesel::update(ingredients::table.find(local_id))
            .set((
                ingredients::name.eq(&row.name),
                ingredients::name_key.eq(&row.name_key),
                ingredients::notes.eq(&row.notes),
                ingredients::basis.eq(row.basis),
                ingredients::calories.eq(row.calories),
                ingredients::protein.eq(row.protein),
                ingredients::carbohydrates.eq(row.carbohydrates),
                ingredients::fat.eq(row.fat),
            ))
            .execute(connection)?;
        diesel::delete(
            ingredient_units::table.filter(ingredient_units::ingredient_local_id.eq(local_id)),
        )
        .execute(connection)?;
        local_id
    } else {
        diesel::insert_into(ingredients::table)
            .values(&row)
            .execute(connection)?;
        ingredients::table
            .filter(ingredients::remote_id.eq(RemoteIngredientId::from(id)))
            .select(ingredients::local_id)
            .first::<LocalIngredientId>(connection)?
    };
    insert_units(connection, local_id, &data)?;
    record(connection, local_id)
}

fn insert_units(
    connection: &mut SqliteConnection,
    parent: LocalIngredientId,
    data: &IngredientWriteData,
) -> Result<(), StoreFailure> {
    for (position, unit) in data.units().iter().enumerate() {
        let row = UnitRow::from_domain(parent, position, unit)?;
        diesel::insert_into(ingredient_units::table)
            .values(&row)
            .execute(connection)?;
    }
    Ok(())
}

#[async_trait::async_trait]
impl IngredientStoreMut for SqliteDatabase {
    #[tracing::instrument(skip_all)]
    async fn list(&mut self) -> Result<Vec<Ingredient>, IngredientLoadError> {
        let result = self
            .connection
            .spawn_blocking(|connection| Ok(connection.transaction::<_, StoreFailure, _>(records)))
            .await;
        match result {
            Ok(Ok(records)) => Ok(records),
            error => {
                tracing::error!(?error, "Ingredient list failed");
                Err(IngredientLoadError::Storage)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn save(&mut self, write: IngredientWrite) -> Result<Ingredient, IngredientSaveError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(connection
                    .immediate_transaction(|connection| save_in_transaction(connection, write)))
            })
            .await;
        match result {
            Ok(Ok(saved)) => Ok(saved),
            Ok(Err(StoreFailure::DuplicateName)) => Err(IngredientSaveError::DuplicateName),
            Ok(Err(StoreFailure::NotFound)) => Err(IngredientSaveError::NotFound),
            error => {
                tracing::error!(?error, "Ingredient save failed");
                Err(IngredientSaveError::Storage)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn delete(&mut self, id: IngredientId) -> Result<(), IngredientDeleteError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(
                    connection.immediate_transaction::<_, StoreFailure, _>(|connection| {
                        diesel::delete(
                            ingredients::table
                                .filter(ingredients::remote_id.eq(RemoteIngredientId::from(id))),
                        )
                        .execute(connection)?;
                        Ok(())
                    }),
                )
            })
            .await;
        match result {
            Ok(Ok(())) => Ok(()),
            error => {
                tracing::error!(?error, "Ingredient delete failed");
                Err(IngredientDeleteError::Storage)
            }
        }
    }
}

#[cfg(test)]
mod tests;
