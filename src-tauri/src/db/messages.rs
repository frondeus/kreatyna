use std::sync::mpsc::Sender;

pub enum UiToDb {
    CreateDocument {
        doc: super::models::NewDocument, 
        response: Sender<i32>
    },
}
