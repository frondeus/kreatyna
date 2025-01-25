use super::messages::UiToDb;
use super::schema::documents;
use anyhow::Result;
use diesel::prelude::*;

pub fn handle(msg: UiToDb, conn: &mut SqliteConnection) -> Result<()> {
    match msg {
        UiToDb::CreateDocument { doc, response } => {
            let id = diesel::insert_into(documents::table)
                .values(&doc)
                .returning(documents::id)
                .get_result(conn)?;
            response.send(id)?;
        }
    }
    Ok(())
}
