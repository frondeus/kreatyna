use super::messages::UiToDb;
use super::models::{Document, NewDocument};
use super::schema::documents;
use anyhow::Result;
use diesel::prelude::*;

pub fn handle(msg: UiToDb, conn: &mut SqliteConnection) -> Result<()> {
    println!("Handling message: {:?}", msg);
    match msg {
        UiToDb::CreateDocument { doc, response } => {
            let id = diesel::insert_into(documents::table)
                .values(&doc)
                .returning(documents::id)
                .get_result(conn)?;
            response.send(id)?;
        }
        UiToDb::GetDocuments { kind, response } => {
            let docs = Document::by_kind(kind).load(conn)?;
            response.send(docs)?;
        }
        UiToDb::GetDocument { id, response } => {
            let doc = Document::by_id(id).first(conn)?;
            response.send(doc)?;
        }
        UiToDb::UpdateDocument { doc } => {
            diesel::update(Document::by_id(doc.id))
                .set(&doc)
                .execute(conn)?;
        } // UiToDb::NewEvent {
          //     for_document,
          //     event,
          // } => {
          //     todo!()
          // }
    }
    Ok(())
}
