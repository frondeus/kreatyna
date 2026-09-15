use std::{collections::BTreeMap, fmt, sync::Arc};

use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
    span,
};
use tracing_subscriber::{
    Layer, filter::LevelFilter, layer::Context, prelude::*, registry::LookupSpan,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct LogField {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct LogSpan {
    pub name: String,
    pub target: String,
    pub fields: Vec<LogField>,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct LogRecord {
    pub level: LogLevel,
    pub target: String,
    pub message: Option<String>,
    pub fields: Vec<LogField>,
    pub spans: Vec<LogSpan>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[uniffi::export(foreign)]
pub trait LoggerHandler: Send + Sync {
    fn log(&self, record: LogRecord);
}

pub struct LoggerLayer {
    handler: Arc<dyn LoggerHandler>,
}

impl LoggerLayer {
    pub fn new(handler: Arc<dyn LoggerHandler>) -> Self {
        Self { handler }
    }
}

impl From<&tracing::Level> for LogLevel {
    fn from(level: &tracing::Level) -> Self {
        match *level {
            tracing::Level::TRACE => Self::Trace,
            tracing::Level::DEBUG => Self::Debug,
            tracing::Level::INFO => Self::Info,
            tracing::Level::WARN => Self::Warn,
            tracing::Level::ERROR => Self::Error,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Self::TRACE,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum LoggingInitError {
    #[error("A global tracing subscriber is already installed")]
    AlreadyInitialized,
}

#[uniffi::export]
pub fn initialize_logging(
    handler: Arc<dyn LoggerHandler>,
    minimum_level: LogLevel,
) -> Result<(), LoggingInitError> {
    let subscriber = tracing_subscriber::registry()
        .with(LoggerLayer::new(handler).with_filter(LevelFilter::from(minimum_level)));
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| LoggingInitError::AlreadyInitialized)
}

#[derive(Default)]
struct StoredFields(BTreeMap<String, String>);

impl Visit for StoredFields {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_owned(), value.to_owned());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0.insert(field.name().to_owned(), format!("{value:?}"));
    }
}

impl StoredFields {
    fn into_fields(self) -> Vec<LogField> {
        self.0
            .into_iter()
            .map(|(name, value)| LogField { name, value })
            .collect()
    }

    fn fields(&self) -> Vec<LogField> {
        self.0
            .iter()
            .map(|(name, value)| LogField {
                name: name.clone(),
                value: value.clone(),
            })
            .collect()
    }
}

impl<S> Layer<S> for LoggerLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let mut fields = StoredFields::default();
        attrs.record(&mut fields);
        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(fields);
        }
    }

    fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>) {
        // Format before taking the extensions lock: user Debug implementations
        // may perform arbitrary work.
        let mut updates = StoredFields::default();
        values.record(&mut updates);
        if let Some(span) = ctx.span(id) {
            let mut extensions = span.extensions_mut();
            if let Some(fields) = extensions.get_mut::<StoredFields>() {
                fields.0.extend(updates.0);
            } else {
                extensions.insert(updates);
            }
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut fields = StoredFields::default();
        event.record(&mut fields);
        let message = fields.0.remove("message");
        let mut spans = Vec::new();
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let fields = span
                    .extensions()
                    .get::<StoredFields>()
                    .map(StoredFields::fields)
                    .unwrap_or_default();
                spans.push(LogSpan {
                    name: span.name().to_owned(),
                    target: span.metadata().target().to_owned(),
                    fields,
                });
            }
        }

        // All extension guards are dropped before crossing the foreign boundary.
        self.handler.log(LogRecord {
            level: metadata.level().into(),
            target: metadata.target().to_owned(),
            message,
            fields: fields.into_fields(),
            spans,
            file: metadata.file().map(str::to_owned),
            line: metadata.line(),
        });
    }
}

#[cfg(test)]
mod tests;
