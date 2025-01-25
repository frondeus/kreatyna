use std::sync::mpsc::Sender;

use changes::ApplyChange;
use db::messages::UiToDbExt;
use ui::map::{FromDb, ToDb};

mod changes;
mod db;
mod net;
mod ui;

#[tauri::command]
fn create_document(
    new_doc: ui::models::NewDocument,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> ui::models::DocumentId {
    let id = tx.create_document(new_doc.to_db().unwrap());
    ui::models::DocumentId::from(id)
}

#[tauri::command]
fn update_document(
    id: ui::models::DocumentId,
    changes: Vec<(ui::models::JsonPath, net::models::DocumentEventChange)>,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
    notify: tauri::State<Sender<ui::models::EventNotification>>,
) {
    println!("update_document: {:?} {:?}", id, changes);
    let doc = tx.get_document(id.into());
    let mut doc: ui::models::Document = FromDb::from_db(doc).unwrap();
    let mut ui_changes = vec![];
    for (json_path, change) in changes {
        let path = json_path.as_ref().split('.').collect::<Vec<_>>();
        if let Some(ui) = doc.apply_change(&path, &change) {
            ui_changes.push((json_path, ui));
        }
    }
    tx.update_document(doc.to_db().unwrap());
    notify
        .send(ui::models::EventNotification::Updated {
            id: id.into(),
            changes: ui_changes,
        })
        .unwrap();
}

#[tauri::command]
fn get_documents(
    kind: ui::models::JustDocumentKind,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Vec<ui::models::Document> {
    println!("get_documents: {:?}", kind);
    let docs = tx.get_documents(kind.to_string());
    docs.into_iter()
        .map(|d| FromDb::from_db(d).unwrap())
        .collect()
}

// Specialized commands outside of the event document model

#[tauri::command]
fn get_items_and_suggestions(
    query: String,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Vec<ui::models::AddShoppingListItem> {
    println!("get_item_suggestions: {}", query);
    let _ = tx;
    vec![]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (db_tx, db_rx) = std::sync::mpsc::channel();
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || db::task(db_rx));
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db_tx)
        .manage(ui_tx)
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || ui::task(ui_rx, handle));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_document,
            get_documents,
            update_document,
            get_items_and_suggestions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
