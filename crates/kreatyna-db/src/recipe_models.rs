//! Typed SQLite rows for recipes, their recipe ingredients and their steps,
//! and their conversion to the domain types. `recipe_store` reads and writes them.
use diesel::{
    AsExpression, FromSqlRow, Insertable, Queryable, Selectable,
    deserialize::FromSql,
    serialize::{IsNull, Output, ToSql},
    sql_types::{BigInt, Text},
    sqlite::{Sqlite, SqliteValue},
};
use kreatyna::{
    ingredients::name_key,
    recipes::{
        CanvasPoint, KnownUnit, Recipe, RecipeId, RecipeIngredient, RecipeIngredientAmount,
        RecipeIngredientId, RecipeIngredientName, RecipeIngredientNote, RecipeIngredientQuantity,
        RecipeTitle, Step, StepDurationS, StepEdges, StepId, StepInstructions, StepKind, StepWrite,
    },
};

use super::{
    columns::{db_decimal, db_enum, db_text},
    schema::{recipe_ingredient_lines, recipe_step_edges, recipe_steps, recipes},
};

/// The `local_id` of a `recipes` row: the key that its steps point to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct LocalRecipeId(pub i64);

impl FromSql<BigInt, Sqlite> for LocalRecipeId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        i64::from_sql(value).map(Self)
    }
}

impl ToSql<BigInt, Sqlite> for LocalRecipeId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

/// The `local_id` of a `recipe_steps` row: the key that its edges point to
/// (REC-18).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct LocalStepId(pub i64);

impl FromSql<BigInt, Sqlite> for LocalStepId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        i64::from_sql(value).map(Self)
    }
}

impl ToSql<BigInt, Sqlite> for LocalStepId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

/// A row's place in its recipe's list: 0 for the first step or line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct DbOrdinal(i64);

impl DbOrdinal {
    /// 0, 1, 2, … for the steps or the lines of a recipe, in list order.
    pub fn sequence() -> impl Iterator<Item = Self> {
        (0..).map(Self)
    }
}

impl FromSql<BigInt, Sqlite> for DbOrdinal {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        i64::from_sql(value).map(Self)
    }
}

impl ToSql<BigInt, Sqlite> for DbOrdinal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

/// One coordinate of a canvas point. The column is a `BIGINT`; reading a
/// value that does not fit a `u32` fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct DbCoordinate(pub u32);

impl FromSql<BigInt, Sqlite> for DbCoordinate {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        Ok(Self(u32::try_from(i64::from_sql(value)?)?))
    }
}

impl ToSql<BigInt, Sqlite> for DbCoordinate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(i64::from(self.0));
        Ok(IsNull::No)
    }
}

/// A step's duration in seconds (REC-17). Reading a value outside 1 second
/// to 365 days fails, as the table's CHECK does not allow it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct DbDurationS(pub StepDurationS);

impl FromSql<BigInt, Sqlite> for DbDurationS {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let seconds = u32::try_from(i64::from_sql(value)?)?;
        Ok(Self(StepDurationS::try_new(seconds)?))
    }
}

impl ToSql<BigInt, Sqlite> for DbDurationS {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(i64::from(self.0.into_inner()));
        Ok(IsNull::No)
    }
}

// A step's kind as its text (REC-17). Any other text fails to read.
db_enum!(DbStepKind, StepKind {
    Active => "active",
    Waiting => "waiting",
});

// A step's instructions (REC-17), and a line's name (REC-07) and note
// (REC-08). Reading a blank text fails, as the tables' CHECKs do not allow it.
db_text!(DbInstructions, StepInstructions);
db_text!(DbRecipeIngredientName, RecipeIngredientName);
db_text!(DbRecipeIngredientNote, RecipeIngredientNote);

// A line's amount as exact decimal text (REC-08). Reading text that is not a
// decimal more than 0 fails.
db_decimal!(DbRecipeIngredientAmount, RecipeIngredientAmount);

// A line's known unit as its symbol (REC-08). Any other text fails to read,
// as the table's CHECK does not allow it.
db_enum!(DbKnownUnit, KnownUnit {
    Gram => "g",
    Kilogram => "kg",
    Millilitre => "ml",
    Litre => "l",
    Teaspoon => "tsp",
    Tablespoon => "tbsp",
    Cup => "cup",
    Piece => "piece",
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct RemoteRecipeId(pub RecipeId);

impl FromSql<Text, Sqlite> for RemoteRecipeId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(text.parse()?))
    }
}

impl ToSql<Text, Sqlite> for RemoteRecipeId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct RemoteStepId(pub StepId);

impl FromSql<Text, Sqlite> for RemoteStepId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(text.parse()?))
    }
}

impl ToSql<Text, Sqlite> for RemoteStepId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct RemoteRecipeIngredientId(pub RecipeIngredientId);

impl FromSql<Text, Sqlite> for RemoteRecipeIngredientId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(text.parse()?))
    }
}

impl ToSql<Text, Sqlite> for RemoteRecipeIngredientId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipes)]
pub(crate) struct RecipeRow {
    pub local_id: LocalRecipeId,
    pub remote_id: RemoteRecipeId,
    pub title: String,
    pub title_key: String,
}

impl RecipeRow {
    /// The recipe with `lines` and `steps`, which the store read in list
    /// order, and the steps' `edges`.
    pub fn into_domain(
        self,
        lines: Vec<RecipeIngredient>,
        steps: Vec<Step>,
        edges: StepEdges,
    ) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
            lines,
            steps,
            edges,
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipe_steps)]
pub(crate) struct StepRow {
    /// The key that the step's edges point to. `into_domain` drops it.
    pub local_id: LocalStepId,
    pub remote_id: RemoteStepId,
    pub title: String,
    pub x: DbCoordinate,
    pub y: DbCoordinate,
    pub duration_seconds: Option<DbDurationS>,
    pub kind: DbStepKind,
    pub instructions: Option<DbInstructions>,
}

impl StepRow {
    pub fn into_domain(self) -> Step {
        Step {
            id: self.remote_id.0,
            title: self.title,
            position: CanvasPoint {
                x: self.x.0,
                y: self.y.0,
            },
            duration: self.duration_seconds.map(|duration| duration.0),
            kind: self.kind.0,
            instructions: self.instructions.map(|instructions| instructions.0),
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipe_steps)]
pub(crate) struct NewStepRow {
    pub remote_id: RemoteStepId,
    pub recipe_local_id: LocalRecipeId,
    pub ordinal: DbOrdinal,
    pub title: String,
    pub x: DbCoordinate,
    pub y: DbCoordinate,
    pub duration_seconds: Option<DbDurationS>,
    pub kind: DbStepKind,
    pub instructions: Option<DbInstructions>,
}

impl NewStepRow {
    /// The step at `ordinal` (the order added) of the recipe row
    /// `recipe_local_id`.
    pub fn new(recipe_local_id: LocalRecipeId, ordinal: DbOrdinal, step: &StepWrite) -> Self {
        Self {
            remote_id: RemoteStepId(step.id),
            recipe_local_id,
            ordinal,
            title: step.title.as_ref().to_owned(),
            x: DbCoordinate(step.position.x),
            y: DbCoordinate(step.position.y),
            duration_seconds: step.duration.map(DbDurationS),
            kind: DbStepKind(step.kind),
            instructions: step.instructions.clone().map(DbInstructions),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipe_ingredient_lines)]
pub(crate) struct RecipeIngredientRow {
    pub remote_id: RemoteRecipeIngredientId,
    pub name: DbRecipeIngredientName,
    pub amount: Option<DbRecipeIngredientAmount>,
    pub unit: Option<DbKnownUnit>,
    pub note: Option<DbRecipeIngredientNote>,
}

/// A stored line has a unit and no amount. Only a broken file or another
/// program can write it: the table's CHECK refuses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("a stored recipe ingredient has a unit and no amount")]
pub(crate) struct UnitWithoutAmount;

impl RecipeIngredientRow {
    pub fn into_domain(self) -> Result<RecipeIngredient, UnitWithoutAmount> {
        let quantity = match (self.amount, self.unit) {
            (None, None) => RecipeIngredientQuantity::Unmeasured,
            (Some(amount), None) => RecipeIngredientQuantity::Count(amount.0),
            (Some(amount), Some(unit)) => RecipeIngredientQuantity::Measured {
                amount: amount.0,
                unit: unit.0,
            },
            (None, Some(_)) => return Err(UnitWithoutAmount),
        };
        Ok(RecipeIngredient {
            id: self.remote_id.0,
            name: self.name.0,
            quantity,
            note: self.note.map(|note| note.0),
        })
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipe_ingredient_lines)]
pub(crate) struct NewRecipeIngredientRow {
    pub remote_id: RemoteRecipeIngredientId,
    pub recipe_local_id: LocalRecipeId,
    pub ordinal: DbOrdinal,
    pub name: DbRecipeIngredientName,
    pub amount: Option<DbRecipeIngredientAmount>,
    pub unit: Option<DbKnownUnit>,
    pub note: Option<DbRecipeIngredientNote>,
}

impl NewRecipeIngredientRow {
    /// The line at `ordinal` (its place in the list) of the recipe row
    /// `recipe_local_id`.
    pub fn new(
        recipe_local_id: LocalRecipeId,
        ordinal: DbOrdinal,
        line: &RecipeIngredient,
    ) -> Self {
        Self {
            remote_id: RemoteRecipeIngredientId(line.id),
            recipe_local_id,
            ordinal,
            name: DbRecipeIngredientName(line.name.clone()),
            amount: match line.quantity {
                RecipeIngredientQuantity::Unmeasured => None,
                RecipeIngredientQuantity::Count(amount)
                | RecipeIngredientQuantity::Measured { amount, .. } => {
                    Some(DbRecipeIngredientAmount(amount))
                }
            },
            unit: match line.quantity {
                RecipeIngredientQuantity::Unmeasured | RecipeIngredientQuantity::Count(_) => None,
                RecipeIngredientQuantity::Measured { unit, .. } => Some(DbKnownUnit(unit)),
            },
            note: line.note.clone().map(DbRecipeIngredientNote),
        }
    }
}

/// One `recipe_step_edges` row: "`to` comes after `from`", by the steps'
/// row keys (REC-18).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Queryable, Selectable, Insertable)]
#[diesel(table_name = recipe_step_edges)]
pub(crate) struct StepEdgeRow {
    pub from_local_id: LocalStepId,
    pub to_local_id: LocalStepId,
}

/// The step that the store saved for `step`.
pub(crate) fn saved_step(step: &StepWrite) -> Step {
    Step {
        id: step.id,
        title: step.title.as_ref().to_owned(),
        position: step.position,
        duration: step.duration,
        kind: step.kind,
        instructions: step.instructions.clone(),
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipes)]
pub(crate) struct NewRecipeRow {
    pub remote_id: RemoteRecipeId,
    pub title: String,
    pub title_key: String,
}

impl NewRecipeRow {
    pub fn new(id: RecipeId, title: &RecipeTitle) -> Self {
        Self {
            remote_id: RemoteRecipeId(id),
            title: title.as_ref().to_owned(),
            title_key: name_key(title.as_ref()),
        }
    }

    /// The saved recipe with `lines`, `steps` and `edges`, as the store wrote
    /// them.
    pub fn into_domain(
        self,
        lines: Vec<RecipeIngredient>,
        steps: Vec<Step>,
        edges: StepEdges,
    ) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
            lines,
            steps,
            edges,
        }
    }
}
