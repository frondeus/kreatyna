use std::sync::mpsc::Receiver;

use anyhow::Result;
use diesel::prelude::*;
use messages::UiToDb;
use schema::documents;

mod schema;
pub mod models;
mod dsl {

}
pub mod messages {
    pub enum UiToDb {
        CreateDocument(super::models::NewDocument)
    }
}

pub fn task(rx: Receiver<UiToDb>) -> anyhow::Result<()> {
    let mut conn = establish_connection()?;
    loop {
        let msg = rx.recv()?;
        if let Err(e) = handle(msg, &mut conn) {
            eprintln!("Error handling message: {}", e);
        }
    }
}

fn handle(msg: UiToDb, conn: &mut SqliteConnection) -> Result<()> {
    match msg {
        UiToDb::CreateDocument(doc) => {
            diesel::insert_into(documents::table).values(&doc).execute(conn)?;
        }
    }
    Ok(())
}

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = "./db.sqlite";

    let conn = SqliteConnection::establish(&database_url)?;
    Ok(conn)
}
