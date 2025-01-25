#![allow(dead_code)]

use crate::db::schema::{documents, events};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(Sqlite))]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub kind: String,
    pub body: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub title: String,
    pub kind: String,
    pub body: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub creator_id: i32,
    pub creator_counter: i32,
    pub vector_clock: String,
    pub document_id: i32,
    pub kind: String,
    pub data: String,
}
