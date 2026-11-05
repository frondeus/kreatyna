//! Typed SQLite rows for recipes, their ingredient lines and their steps,
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
        CanvasPoint, IngredientLine, IngredientLineId, IngredientLineName, Recipe, RecipeId,
        RecipeTitle, Step, StepDurationS, StepEdges, StepId, StepInstructions, StepKind, StepWrite,
    },
};

use super::schema::{recipe_ingredient_lines, recipe_step_edges, recipe_steps, recipes};

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

/// An ingredient line's name (REC-07). Reading a blank text fails, as the
/// table's CHECK does not allow it.
#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct DbLineName(pub IngredientLineName);

impl FromSql<Text, Sqlite> for DbLineName {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(IngredientLineName::try_new(text)?))
    }
}

impl ToSql<Text, Sqlite> for DbLineName {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub(crate) struct RemoteLineId(pub IngredientLineId);

impl FromSql<Text, Sqlite> for RemoteLineId {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let text = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Ok(Self(text.parse()?))
    }
}

impl ToSql<Text, Sqlite> for RemoteLineId {
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
        lines: Vec<IngredientLine>,
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
pub(crate) struct LineRow {
    pub remote_id: RemoteLineId,
    pub name: DbLineName,
}

impl LineRow {
    pub fn into_domain(self) -> IngredientLine {
        IngredientLine {
            id: self.remote_id.0,
            name: self.name.0,
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipe_ingredient_lines)]
pub(crate) struct NewLineRow {
    pub remote_id: RemoteLineId,
    pub recipe_local_id: LocalRecipeId,
    pub ordinal: DbOrdinal,
    pub name: DbLineName,
}

impl NewLineRow {
    /// The line at `ordinal` (its place in the list) of the recipe row
    /// `recipe_local_id`.
    pub fn new(recipe_local_id: LocalRecipeId, ordinal: DbOrdinal, line: &IngredientLine) -> Self {
        Self {
            remote_id: RemoteLineId(line.id),
            recipe_local_id,
            ordinal,
            name: DbLineName(line.name.clone()),
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
        lines: Vec<IngredientLine>,
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
