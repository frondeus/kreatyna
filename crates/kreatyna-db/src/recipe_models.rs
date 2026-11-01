//! Typed SQLite rows for recipes and their conversion to the domain type.
//! `recipe_store` reads and writes them.
use diesel::{
    AsExpression, FromSqlRow, Insertable, Queryable, Selectable,
    deserialize::FromSql,
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use kreatyna::{
    ingredients::name_key,
    recipes::{Recipe, RecipeId, RecipeTitle},
};

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

    pub fn into_domain(self) -> Recipe {
        Recipe {
            id: self.remote_id.0,
            title: self.title,
        }
    }
}
