use std::sync::mpsc::Receiver;

use anyhow::Result;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use messages::UiToDb;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod dsl;
mod handler;
pub mod messages;
pub mod models;
mod schema;

pub fn task(rx: Receiver<UiToDb>) -> anyhow::Result<()> {
    let mut conn = establish_connection()?;
    loop {
        let msg = rx.recv()?;
        if let Err(e) = handler::handle(msg, &mut conn) {
            eprintln!("Error handling message: {}", e);
        }
    }
}

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = "./db.sqlite";

    let mut conn = SqliteConnection::establish(database_url)?;

    if conn.run_pending_migrations(MIGRATIONS).is_err() {
        anyhow::bail!("Failed to run migrations");
    }

    Ok(conn)
}
