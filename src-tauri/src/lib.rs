use std::sync::mpsc::Sender;

use ui::map::ToDb;

mod ui;
mod db;

#[tauri::command]
fn create_document(
    new_doc: ui::models::Document,
    tx: tauri::State<Sender<db::messages::UiToDb>>
) {
    tx.send(db::messages::UiToDb::CreateDocument(
        new_doc.to_db().unwrap(),
    ))
    .unwrap();
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
