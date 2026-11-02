//! Typed SQLite rows for ingredients and their reference units, and their
//! conversion to and from the domain types. `ingredient_store` reads and writes
//! them.
use diesel::{
    AsExpression, FromSqlRow, Insertable, Queryable, Selectable,
    deserialize::FromSql,
    serialize::{IsNull, Output, ToSql},
    sql_types::{BigInt, Integer, Text},
    sqlite::{Sqlite, SqliteValue},
};
use kreatyna::ingredients::{
    Grams, Ingredient, IngredientId, IngredientName, IngredientWriteData, Kilocalories,
    Millilitres, Nutrition, NutritionBasis, ReferenceUnit, UnitName, name_key,
};
use rust_decimal::Decimal;

use super::schema::{ingredient_units, ingredients};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum ModelError {
    #[error("invalid ingredient basis: {0}")]
    Basis(i32),
    #[error("reference measurement must be positive")]
    NonPositiveMeasurement,
    #[error("reference unit needs a measurement")]
    MissingMeasurement,
    #[error("unit position cannot fit in SQLite")]
    PositionOverflow,
    #[error("negative unit position")]
    NegativePosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct LocalIngredientId(pub i64);

impl FromSql<BigInt, Sqlite> for LocalIngredientId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        i64::from_sql(value).map(Self)
    }
}

impl ToSql<BigInt, Sqlite> for LocalIngredientId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct UnitPosition(i64);

impl TryFrom<usize> for UnitPosition {
    type Error = ModelError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        i64::try_from(value)
            .map(Self)
            .map_err(|_| ModelError::PositionOverflow)
    }
}

impl TryFrom<UnitPosition> for usize {
    type Error = ModelError;

    fn try_from(value: UnitPosition) -> Result<Self, Self::Error> {
        if value.0 < 0 {
            return Err(ModelError::NegativePosition);
        }
        usize::try_from(value.0).map_err(|_| ModelError::PositionOverflow)
    }
}

impl FromSql<BigInt, Sqlite> for UnitPosition {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let raw = i64::from_sql(value)?;
        usize::try_from(raw).map_err(|_| ModelError::NegativePosition)?;
        Ok(Self(raw))
    }
}

impl ToSql<BigInt, Sqlite> for UnitPosition {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        usize::try_from(self.0).map_err(|_| ModelError::NegativePosition)?;
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct RemoteIngredientId(pub IngredientId);

impl From<IngredientId> for RemoteIngredientId {
    fn from(value: IngredientId) -> Self {
        Self(value)
    }
}

impl From<RemoteIngredientId> for IngredientId {
    fn from(value: RemoteIngredientId) -> Self {
        value.0
    }
}

impl FromSql<Text, Sqlite> for RemoteIngredientId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(text.parse()?))
    }
}

impl ToSql<Text, Sqlite> for RemoteIngredientId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub(crate) struct DbNutritionBasis(pub NutritionBasis);

impl From<NutritionBasis> for DbNutritionBasis {
    fn from(value: NutritionBasis) -> Self {
        Self(value)
    }
}

impl From<DbNutritionBasis> for NutritionBasis {
    fn from(value: DbNutritionBasis) -> Self {
        value.0
    }
}

impl TryFrom<i32> for DbNutritionBasis {
    type Error = ModelError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self(NutritionBasis::Per100Grams)),
            1 => Ok(Self(NutritionBasis::Per100Millilitres)),
            _ => Err(ModelError::Basis(value)),
        }
    }
}

impl From<DbNutritionBasis> for i32 {
    fn from(value: DbNutritionBasis) -> Self {
        match value.0 {
            NutritionBasis::Per100Grams => 0,
            NutritionBasis::Per100Millilitres => 1,
        }
    }
}

impl FromSql<Integer, Sqlite> for DbNutritionBasis {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        Self::try_from(i32::from_sql(value)?).map_err(Into::into)
    }
}

impl ToSql<Integer, Sqlite> for DbNutritionBasis {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(i32::from(*self));
        Ok(IsNull::No)
    }
}

macro_rules! db_quantity {
    ($name:ident, $domain:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
        #[diesel(sql_type = Text)]
        pub(crate) struct $name(pub $domain);

        impl From<$domain> for $name {
            fn from(value: $domain) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $domain {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl FromSql<Text, Sqlite> for $name {
            fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
                let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
                let decimal = Decimal::from_str_exact(&text)?;
                Ok(Self(<$domain>::try_new(decimal)?))
            }
        }

        impl ToSql<Text, Sqlite> for $name {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
                out.set_value(Decimal::from(self.0).to_string());
                Ok(IsNull::No)
            }
        }
    };
}

db_quantity!(DbCalories, Kilocalories);
db_quantity!(DbGrams, Grams);
db_quantity!(DbMillilitres, Millilitres);

/// A name column. Reading a blank name fails, because the domain type rejects it.
macro_rules! db_name {
    ($name:ident, $domain:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
        #[diesel(sql_type = Text)]
        pub(crate) struct $name(pub $domain);

        impl From<$domain> for $name {
            fn from(value: $domain) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $domain {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl FromSql<Text, Sqlite> for $name {
            fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
                let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
                Ok(Self(<$domain>::try_new(text)?))
            }
        }

        impl ToSql<Text, Sqlite> for $name {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
                out.set_value(self.0.as_ref());
                Ok(IsNull::No)
            }
        }
    };
}

db_name!(DbIngredientName, IngredientName);
db_name!(DbUnitName, UnitName);

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = ingredients)]
pub(crate) struct IngredientRow {
    pub local_id: LocalIngredientId,
    pub remote_id: RemoteIngredientId,
    pub name: DbIngredientName,
    pub name_key: String,
    pub notes: Option<String>,
    pub basis: DbNutritionBasis,
    pub calories: Option<DbCalories>,
    pub protein: Option<DbGrams>,
    pub carbohydrates: Option<DbGrams>,
    pub fat: Option<DbGrams>,
}

impl IngredientRow {
    pub fn into_domain(self, units: Vec<ReferenceUnit>) -> Ingredient {
        Ingredient {
            id: self.remote_id.into(),
            name: self.name.into(),
            notes: self.notes,
            nutrition: Nutrition {
                basis: self.basis.into(),
                calories: self.calories.map(Into::into),
                protein: self.protein.map(Into::into),
                carbohydrates: self.carbohydrates.map(Into::into),
                fat: self.fat.map(Into::into),
            },
            units,
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = ingredients)]
pub(crate) struct NewIngredientRow {
    pub remote_id: RemoteIngredientId,
    pub name: DbIngredientName,
    pub name_key: String,
    pub notes: Option<String>,
    pub basis: DbNutritionBasis,
    pub calories: Option<DbCalories>,
    pub protein: Option<DbGrams>,
    pub carbohydrates: Option<DbGrams>,
    pub fat: Option<DbGrams>,
}

impl NewIngredientRow {
    pub fn from_write(id: IngredientId, data: &IngredientWriteData) -> Self {
        let nutrition = data.nutrition();
        Self {
            remote_id: id.into(),
            name: data.name().clone().into(),
            name_key: name_key(data.name().as_ref()),
            notes: data.notes().map(str::to_owned),
            basis: nutrition.basis.into(),
            calories: nutrition.calories.map(Into::into),
            protein: nutrition.protein.map(Into::into),
            carbohydrates: nutrition.carbohydrates.map(Into::into),
            fat: nutrition.fat.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Insertable)]
#[diesel(table_name = ingredient_units)]
pub(crate) struct UnitRow {
    pub ingredient_local_id: LocalIngredientId,
    pub position: UnitPosition,
    pub name: DbUnitName,
    pub name_key: String,
    pub grams: Option<DbGrams>,
    pub millilitres: Option<DbMillilitres>,
}

impl UnitRow {
    pub fn from_domain(
        parent: LocalIngredientId,
        position: usize,
        unit: &ReferenceUnit,
    ) -> Result<Self, ModelError> {
        if unit.grams.is_none() && unit.millilitres.is_none() {
            return Err(ModelError::MissingMeasurement);
        }
        if unit
            .grams
            .is_some_and(|value| Decimal::from(value) <= Decimal::ZERO)
            || unit
                .millilitres
                .is_some_and(|value| Decimal::from(value) <= Decimal::ZERO)
        {
            return Err(ModelError::NonPositiveMeasurement);
        }
        Ok(Self {
            ingredient_local_id: parent,
            position: position.try_into()?,
            name: unit.name.clone().into(),
            name_key: name_key(unit.name.as_ref()),
            grams: unit.grams.map(Into::into),
            millilitres: unit.millilitres.map(Into::into),
        })
    }

    pub fn into_domain(self) -> Result<(usize, ReferenceUnit), ModelError> {
        if self.grams.is_none() && self.millilitres.is_none() {
            return Err(ModelError::MissingMeasurement);
        }
        if self
            .grams
            .is_some_and(|value| Decimal::from(value.0) <= Decimal::ZERO)
            || self
                .millilitres
                .is_some_and(|value| Decimal::from(value.0) <= Decimal::ZERO)
        {
            return Err(ModelError::NonPositiveMeasurement);
        }
        Ok((
            self.position.try_into()?,
            ReferenceUnit {
                name: self.name.into(),
                grams: self.grams.map(Into::into),
                millilitres: self.millilitres.map(Into::into),
            },
        ))
    }
}

#[cfg(test)]
mod tests;
