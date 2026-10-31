//! Typed SQLite rows for recipes and their conversion to the domain type.
//! `recipe_store` reads them.
use diesel::{
    AsExpression, FromSqlRow, Queryable, Selectable,
    deserialize::FromSql,
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use kreatyna::recipes::{Recipe, RecipeId};

use super::schema::recipes;

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

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = recipes)]
pub(crate) struct RecipeRow {
    pub remote_id: RemoteRecipeId,
    pub title: String,
    pub title_key: String,
}

impl RecipeRow {
    pub fn into_domain(self) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
        }
    }
}
