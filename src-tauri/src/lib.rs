use std::sync::mpsc::Sender;

mod db;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_document(tx: tauri::State<Sender<db::messages::UiToDb>>) {
    tx.send(db::messages::UiToDb::CreateDocument(
        db::models::NewDocument {
            title: "".to_string(),
            path: "".to_string(),
            kind: "".to_string(),
            body: "".to_string(),
        }
    )).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || db::task(rx));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tx)
        .invoke_handler(tauri::generate_handler![
            greet,
            create_document
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
