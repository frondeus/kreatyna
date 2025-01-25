use std::sync::mpsc::Sender;

use ui::map::{FromDb, ToDb};

mod ui;
mod db;

#[tauri::command]
fn create_document(
    new_doc: ui::models::NewDocument,
    tx: tauri::State<Sender<db::messages::UiToDb>>
) -> ui::models::DocumentId {
    let (res_tx, res_rx) = std::sync::mpsc::channel();
    tx.send(db::messages::UiToDb::CreateDocument {
        doc: new_doc.to_db().unwrap(),
        response: res_tx.clone(),
    }).unwrap();
    let id = res_rx.recv().unwrap();
    ui::models::DocumentId::from(id)
}

#[tauri::command]
fn get_documents(
    kind: ui::models::JustDocumentKind,
    tx: tauri::State<Sender<db::messages::UiToDb>>
) -> Vec<ui::models::Document> {
    println!("get_documents: {:?}", kind);
    let (res_tx, res_rx) = std::sync::mpsc::channel();
    tx.send(db::messages::UiToDb::GetDocuments {
        kind: kind.to_string(),
        response: res_tx.clone(),
    }).unwrap();
    let docs = res_rx.recv().unwrap();
        docs.into_iter().map(|d| FromDb::from_db(d).unwrap()).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || db::task(rx));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tx)
        .invoke_handler(tauri::generate_handler![
            create_document,
            get_documents,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
