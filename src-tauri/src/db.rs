use std::sync::mpsc::Receiver;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use messages::UiToDb;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod dsl;
mod handler;
pub mod messages;
pub mod models;
mod schema;

pub fn task(rx: Receiver<UiToDb>, mut conn: SqliteConnection) -> anyhow::Result<()> {
    loop {
        let msg = rx.recv()?;
        if let Err(e) = handler::handle(msg, &mut conn) {
            tracing::error!("Error handling message: {}", e);
        }
    }
}

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection> {
    if std::path::Path::new(database_url).exists() {
        tracing::info!("Database file exists: {:?}", database_url);
    }

    let mut conn = SqliteConnection::establish(database_url)
        .context("Failed to establish SQLITE connection")?;

    if conn.run_pending_migrations(MIGRATIONS).is_err() {
        anyhow::bail!("Failed to run migrations");
    }

    Ok(conn)
}
