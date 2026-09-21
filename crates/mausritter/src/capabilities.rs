use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{Envelope, TransportError};

#[krucyfiks::capability]
pub trait Render: Send + Sync {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}

#[krucyfiks::capability]
pub trait Clock: Send + Sync {
    #[krucyfiks(panic_on_transport)]
    async fn sleep_ms(&self, ms: u64);
}

#[krucyfiks::capability]
pub trait Database: Send + Sync {
    async fn count(&self) -> Result<Option<usize>, DatabaseError>;
    async fn set_count(&self, count: usize) -> Result<(), DatabaseError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Out of range")]
    OutOfRange,
    #[error("Storage")]
    Storage,
    #[error("Transport")]
    Transport,
}

impl From<TransportError> for DatabaseError {
    fn from(error: TransportError) -> Self {
        tracing::error!(?error, "Transport error");
        Self::Transport
    }
}

// ---- App

/// Select a capability explicitly or opt into recursive resolution with `any_route()`.
/// Raw composition enums intentionally do not implement the resolver traits.
///
/// ```compile_fail,E0599
/// use mausritter::capabilities::{CounterEffect, ResolveRender};
/// async fn resolve(effect: CounterEffect) {
///     effect.resolve_render(|| async {}).await.unwrap();
/// }
/// ```
#[derive(Debug, krucyfiks::Effect)]
pub enum CounterEffect {
    Render(Envelope<RenderRequest, RenderResponse>),
    Clock(Envelope<ClockRequest, ClockResponse>),
    Database(Envelope<DatabaseRequest, DatabaseResponse>),
}

#[derive(Clone)]
pub struct CounterCapabilities {
    pub render: Arc<dyn Render>,
    pub clock: Arc<dyn Clock>,
    pub db: Arc<dyn Database>,
}

pub type CounterEffectStream = BoxStream<'static, CounterEffect>;

impl CounterCapabilities {
    pub fn testing() -> (Self, CounterEffectStream) {
        let (render, render_inbox) = RenderClient::new(32, 1);
        let (clock, clock_inbox) = ClockClient::new(32, 1);
        let (db, database_inbox) = DatabaseClient::new(32, 1);

        let streams = vec![
            render_inbox
                .into_stream()
                .map(CounterEffect::Render)
                .boxed(),
            clock_inbox.into_stream().map(CounterEffect::Clock).boxed(),
            database_inbox
                .into_stream()
                .map(CounterEffect::Database)
                .boxed(),
        ];
        let inbox = select_all(streams).boxed();

        (
            Self {
                render: Arc::new(render),
                clock: Arc::new(clock),
                db: Arc::new(db),
            },
            inbox,
        )
    }
}

#[cfg(test)]
mod tests;
