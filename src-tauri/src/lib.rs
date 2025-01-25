use std::sync::mpsc::Sender;

use ui::map::ToDb;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || db::task(rx));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tx)
        .invoke_handler(tauri::generate_handler![create_document])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
