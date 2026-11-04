//! Typed SQLite rows for recipes and their steps, and their conversion to
//! the domain types. `recipe_store` reads and writes them.
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
        CanvasPoint, Recipe, RecipeId, RecipeTitle, Step, StepDurationS, StepId, StepInstructions,
        StepKind, StepWrite,
    },
};

use super::schema::{recipe_steps, recipes};

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

/// A step's place in the order added: 0 for the first step of a recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub(crate) struct StepOrdinal(i64);

impl StepOrdinal {
    /// 0, 1, 2, … for the steps of a recipe in the order added.
    pub fn sequence() -> impl Iterator<Item = Self> {
        (0..).map(Self)
    }
}

impl FromSql<BigInt, Sqlite> for StepOrdinal {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        i64::from_sql(value).map(Self)
    }
}

impl ToSql<BigInt, Sqlite> for StepOrdinal {
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

/// A step's kind as its text: `active` or `waiting` (REC-17). Any other
/// text fails to read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct DbStepKind(pub StepKind);

impl DbStepKind {
    fn text(self) -> &'static str {
        match self.0 {
            StepKind::Active => "active",
            StepKind::Waiting => "waiting",
        }
    }
}

impl FromSql<Text, Sqlite> for DbStepKind {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        match <String as FromSql<Text, Sqlite>>::from_sql(value)?.as_str() {
            "active" => Ok(Self(StepKind::Active)),
            "waiting" => Ok(Self(StepKind::Waiting)),
            other => Err(format!("unknown step kind {other:?}").into()),
        }
    }
}

impl ToSql<Text, Sqlite> for DbStepKind {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.text());
        Ok(IsNull::No)
    }
}

/// A step's instructions (REC-17). Reading a blank text fails, as the
/// table's CHECK does not allow it.
#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct DbInstructions(pub StepInstructions);

impl FromSql<Text, Sqlite> for DbInstructions {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(StepInstructions::try_new(text)?))
    }
}

impl ToSql<Text, Sqlite> for DbInstructions {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.as_ref().to_owned());
        Ok(IsNull::No)
    }
}

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

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipes)]
pub(crate) struct RecipeRow {
    pub local_id: LocalRecipeId,
    pub remote_id: RemoteRecipeId,
    pub title: String,
    pub title_key: String,
}

impl RecipeRow {
    /// The recipe with `steps`, which the store read in the order added.
    pub fn into_domain(self, steps: Vec<Step>) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
            steps,
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipe_steps)]
pub(crate) struct StepRow {
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
    pub ordinal: StepOrdinal,
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
    pub fn new(recipe_local_id: LocalRecipeId, ordinal: StepOrdinal, step: &StepWrite) -> Self {
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

    /// The saved recipe with `steps`, as the store wrote them.
    pub fn into_domain(self, steps: Vec<Step>) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
            steps,
        }
    }
}
