use tauri::{AppHandle, Emitter};

use crate::ui::models::EventNotification;

pub fn handle(msg: EventNotification, app: &AppHandle) -> anyhow::Result<()> {
    app.emit("event", msg)?;
    Ok(())
}
