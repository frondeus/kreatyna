use std::sync::mpsc::Receiver;

use tauri::AppHandle;

pub mod handler;
pub mod map;
pub mod models;

pub fn task(rx: Receiver<models::EventNotification>, app: AppHandle) -> anyhow::Result<()> {
    loop {
        let msg = rx.recv()?;
        if let Err(e) = handler::handle(msg, &app) {
            eprintln!("Error handling message: {}", e);
        }
    }
}
