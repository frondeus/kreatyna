use anyhow::Result;
use std::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub enum UiToDb {
    CreateDocument {
        doc: super::models::NewDocument,
        response: Sender<i32>,
    },
    GetDocuments {
        kind: String,
        response: Sender<Vec<super::models::Document>>,
    },
    GetDocument {
        id: i32,
        response: Sender<super::models::Document>,
    },
    UpdateDocument {
        doc: super::models::Document,
    }, // NewEvent {
       //     for_document: i32,
       //     event: super::models::NewEvent,
       // },
}

pub trait UiToDbExt {
    fn create_document(&self, doc: super::models::NewDocument) -> Result<i32>;
    fn get_documents(&self, kind: String) -> Result<Vec<super::models::Document>>;
    // fn new_event(&self, for_document: i32, event: super::models::NewEvent);
    fn get_document(&self, id: i32) -> Result<super::models::Document>;
    fn update_document(&self, doc: super::models::Document) -> Result<()>;
}

impl UiToDbExt for Sender<UiToDb> {
    fn create_document(&self, doc: super::models::NewDocument) -> Result<i32> {
        let (tx, rx) = channel();
        self.send(UiToDb::CreateDocument { doc, response: tx })?;
        Ok(rx.recv()?)
    }

    fn get_documents(&self, kind: String) -> Result<Vec<super::models::Document>> {
        let (tx, rx) = channel();
        self.send(UiToDb::GetDocuments { kind, response: tx })?;
        Ok(rx.recv()?)
    }

    // fn new_event(&self, for_document: i32, event: super::models::NewEvent) {
    //     self.send(UiToDb::NewEvent {
    //         for_document,
    //         event,
    //     });
    // }

    fn get_document(&self, id: i32) -> Result<super::models::Document> {
        let (tx, rx) = channel();
        self.send(UiToDb::GetDocument { id, response: tx })?;
        Ok(rx.recv()?)
    }

    fn update_document(&self, doc: super::models::Document) -> Result<()> {
        self.send(UiToDb::UpdateDocument { doc })?;
        Ok(())
    }
}
