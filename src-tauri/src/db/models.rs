use crate::db::schema::documents;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(Sqlite))]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub kind: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub title: String,
    pub kind: String,
    pub body: String,
}
