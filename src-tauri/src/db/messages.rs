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
    fn create_document(&self, doc: super::models::NewDocument) -> i32;
    fn get_documents(&self, kind: String) -> Vec<super::models::Document>;
    // fn new_event(&self, for_document: i32, event: super::models::NewEvent);
    fn get_document(&self, id: i32) -> super::models::Document;
    fn update_document(&self, doc: super::models::Document);
}

impl UiToDbExt for Sender<UiToDb> {
    fn create_document(&self, doc: super::models::NewDocument) -> i32 {
        let (tx, rx) = channel();
        self.send(UiToDb::CreateDocument { doc, response: tx })
            .unwrap();
        rx.recv().unwrap()
    }

    fn get_documents(&self, kind: String) -> Vec<super::models::Document> {
        let (tx, rx) = channel();
        self.send(UiToDb::GetDocuments { kind, response: tx })
            .unwrap();
        rx.recv().unwrap()
    }

    // fn new_event(&self, for_document: i32, event: super::models::NewEvent) {
    //     self.send(UiToDb::NewEvent {
    //         for_document,
    //         event,
    //     });
    // }

    fn get_document(&self, id: i32) -> super::models::Document {
        let (tx, rx) = channel();
        self.send(UiToDb::GetDocument { id, response: tx }).unwrap();
        rx.recv().unwrap()
    }

    fn update_document(&self, doc: super::models::Document) {
        self.send(UiToDb::UpdateDocument { doc }).unwrap();
    }
}
