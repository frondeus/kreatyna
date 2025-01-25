use anyhow::Context;
use ordered_float::OrderedFloat;
use std::sync::mpsc::Sender;
use strsim::normalized_levenshtein;
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_fs::FsExt;
use tracing_subscriber::layer::SubscriberExt;

use changes::ApplyChange;
use db::messages::UiToDbExt;
use ui::{
    map::{FromDb, ToDb},
    models::ShoppingListItemState,
};

mod changes;
mod db;
mod net;
mod ui;

enum Error {
    Any(anyhow::Error),
}
impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Any(e)
    }
}
impl Error {
    pub fn from_err<T: std::error::Error + Send + Sync + 'static>(e: T) -> Self {
        Error::Any(anyhow::Error::from(e))
    }

    pub fn context(self, msg: &'static str) -> Self {
        match self {
            Error::Any(e) => Error::Any(e.context(msg)),
        }
    }
}
trait ResultExt<T> {
    fn context(self, msg: &'static str) -> Result<T, Error>;
}
impl<T> ResultExt<T> for Result<T, Error> {
    fn context(self, msg: &'static str) -> Result<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.context(msg)),
        }
    }
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Error::Any(e) => format!("{:?}", e),
        };
        serializer.serialize_str(&s)
    }
}

#[tauri::command]
fn create_document(
    new_doc: ui::models::NewDocument,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Result<ui::models::DocumentId, Error> {
    let id = tx.create_document(new_doc.to_db()?)?;
    Ok(ui::models::DocumentId::from(id))
}

#[tauri::command]
fn update_document(
    id: ui::models::DocumentId,
    changes: Vec<(ui::models::JsonPath, net::models::DocumentEventChange)>,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
    notify: tauri::State<Sender<ui::models::EventNotification>>,
) -> Result<(), Error> {
    tracing::info!("update_document: {:?} {:?}", id, changes);
    let doc = tx
        .get_document(id.into())
        .context("Failed to load document")?;
    let mut doc: ui::models::Document = FromDb::from_db(doc).context("Failed to map document")?;
    let ui_changes = apply_changes(&mut doc, changes)?;
    tx.update_document(doc.to_db()?)?;
    notify
        .send(ui::models::EventNotification::Updated {
            id,
            changes: ui_changes,
        })
        .map_err(Error::from_err)
        .context("Failed to send notification")?;
    Ok(())
}

fn apply_changes(
    doc: &mut ui::models::Document,
    changes: Vec<(ui::models::JsonPath, net::models::DocumentEventChange)>,
) -> Result<Vec<(ui::models::JsonPath, ui::models::DocumentEventChange)>, Error> {
    tracing::info!("doc: {:?}", doc);
    let mut ui_changes = vec![];
    for (path, change) in changes {
        if let Some((path, ui)) = doc.apply(path.as_ref(), &change) {
            ui_changes.push((ui::models::JsonPath::from(path), ui));
        }
    }
    tracing::info!("UI changes: {:?}", ui_changes);
    tracing::info!("After: {:?}", doc);
    Ok(ui_changes)
}

#[tauri::command]
fn get_documents(
    kind: ui::models::JustDocumentKind,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Result<Vec<ui::models::Document>, Error> {
    tracing::info!("get_documents: {:?}", kind);
    let docs = tx
        .get_documents(kind.to_string())
        .context("Failed to load documents")?;
    docs.into_iter()
        .map(|d| {
            FromDb::from_db(d)
                .context("Failed to map document")
                .map_err(Error::from)
        })
        .collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
fn get_document(
    id: ui::models::DocumentId,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Result<ui::models::Document, Error> {
    let db_doc = tx
        .get_document(id.into())
        .context("Failed to load document")?;

    FromDb::from_db(db_doc)
        .context("Failed to map document")
        .map_err(Error::from)
}

// Specialized commands outside of the event document model

#[tauri::command]
fn get_items_and_suggestions(
    query: String,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
) -> Result<Vec<ui::models::AddShoppingListItem>, Error> {
    tracing::info!("get_item_suggestions: {}", query);
    let doc = tx.get_document(1).context("Failed to load document")?;
    let doc: ui::models::Document = FromDb::from_db(doc).context("Failed to map document")?;
    let ui::models::DocumentKind::ShoppingList { items } = doc.kind else {
        return Err(Error::from(anyhow::anyhow!(
            "Document is not a shopping list"
        )));
    };

    let suggestions: Vec<_> = items
        .into_iter()
        .map(ui::models::AddShoppingListItem::Item)
        .collect();

    let mut suggestions_with_score: Vec<_> = suggestions
        .into_iter()
        .map(|s| {
            let score = OrderedFloat(1.0 - normalized_levenshtein(&query, s.name()));
            (score, s)
        })
        .collect();

    suggestions_with_score.sort_by_key(|s| s.0);
    Ok(suggestions_with_score.into_iter().map(|s| s.1).collect())
}

#[tauri::command]
fn finish_shopping(
    id: ui::models::DocumentId,
    tx: tauri::State<Sender<db::messages::UiToDb>>,
    notify: tauri::State<Sender<ui::models::EventNotification>>,
) -> Result<(), Error> {
    let doc = tx
        .get_document(id.into())
        .context("Failed to load document")?;
    let mut doc: ui::models::Document = FromDb::from_db(doc).context("Failed to map document")?;
    let ui::models::DocumentKind::ShoppingList { items } = &doc.kind else {
        return Err(Error::from(anyhow::anyhow!(
            "Document is not a shopping list"
        )));
    };
    let changes = items
        .iter()
        .filter(|item| item.state == ShoppingListItemState::InCart)
        .map(|i| {
            (
                ui::models::JsonPath::from("items".to_owned()),
                net::models::DocumentEventChange::Remove { key: i.id.clone() },
            )
        })
        .collect();

    let ui_changes = apply_changes(&mut doc, changes)?;
    tx.update_document(doc.to_db()?)?;
    notify
        .send(ui::models::EventNotification::Updated {
            id,
            changes: ui_changes,
        })
        .map_err(Error::from_err)
        .context("Failed to send notification")?;

    Ok(())
}

#[cfg(target_os = "ios")]
fn subscriber() -> impl tracing::Subscriber {
    let registry = tracing_subscriber::registry();
    registry.with(tracing_oslog::OsLogger::new(
        "pl.frondeus.kreatyna.app",
        "default",
    ))
}

#[cfg(target_os = "android")]
fn subscriber() -> impl tracing::Subscriber {
    use tracing_logcat::{LogcatMakeWriter, LogcatTag};

    let registry = tracing_subscriber::registry();
    let writer = LogcatMakeWriter::new(LogcatTag::Target).expect("Failed to create writer");
    let collector = tracing_subscriber::fmt::layer().with_writer(writer);
    registry.with(collector)
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
fn subscriber() -> impl tracing::Subscriber {
    let registry = tracing_subscriber::registry();
    registry.with(tracing_subscriber::fmt::layer())
}

#[cfg(target_os = "ios")]
fn base_dir() -> BaseDirectory {
    BaseDirectory::Home
}

#[cfg(not(target_os = "ios"))]
fn base_dir() -> BaseDirectory {
    BaseDirectory::Document
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing::subscriber::set_global_default(subscriber()).expect("failed to set global subscriber");

    tracing::info!("Starting Kreatyna");

    let (db_tx, db_rx) = std::sync::mpsc::channel();
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(db_tx)
        .manage(ui_tx)
        .setup(|app| {
            let path = app
                .path()
                .resolve("kreatyna.sqlite", base_dir())
                .expect("Failed to resolve path");
            tracing::info!("Database path: {:?}", path);
            let scope = app.fs_scope();
            scope.allow_file(&path);
            let path = path.to_str().expect("Failed to convert path to string");
            let db_conn =
                match db::establish_connection(path).context("Failed to establish connection") {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        std::process::exit(1);
                    }
                };

            let handle = app.handle().clone();
            std::thread::spawn(move || db::task(db_rx, db_conn));
            std::thread::spawn(move || ui::task(ui_rx, handle));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_document,
            get_documents,
            get_document,
            update_document,
            get_items_and_suggestions,
            finish_shopping
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
