use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{
    AnyRoute, EffectTree, Envelope, EnvelopeError, RequestInbox, RequestOutbox, TransportError,
    request_channel,
};

#[async_trait::async_trait]
pub trait Render: Send + Sync {
    /// The channel client panics on transport failure. This is the handwritten
    /// equivalent of the future `#[krucyfiks(panic_on_transport)]` policy.
    async fn render(&self);
}

//--- in future derived
#[async_trait::async_trait]
pub trait RenderMut: Send {
    async fn render(&mut self);
}

#[async_trait::async_trait]
pub trait ResolveRender: Send {
    type ResolveError;

    async fn resolve_mut<I>(self, i: &mut I) -> Result<(), Self::ResolveError>
    where
        I: RenderMut + ?Sized;

    async fn resolve_render<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = ()>;
}
#[async_trait::async_trait]
impl<E> ResolveRender for AnyRoute<E>
where
    E: EffectTree + Send,
{
    type ResolveError = EnvelopeError<RenderResponse>;

    async fn resolve_mut<I>(self, i: &mut I) -> Result<(), Self::ResolveError>
    where
        I: RenderMut + ?Sized,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<RenderRequest, RenderResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveRender::resolve_mut(envelope, i).await
    }

    async fn resolve_render<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = ()>,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<RenderRequest, RenderResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveRender::resolve_render(envelope, handle).await
    }
}
#[async_trait::async_trait]
impl ResolveRender for Envelope<RenderRequest, RenderResponse> {
    type ResolveError = EnvelopeError<RenderResponse>;

    async fn resolve_mut<I>(self, i: &mut I) -> Result<(), Self::ResolveError>
    where
        I: RenderMut + ?Sized,
    {
        let (request, responder) = self.into_parts();
        let response = request.dispatch_mut(i).await;

        responder.answer_async(response).await
    }

    async fn resolve_render<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = ()>,
    {
        let (request, response) = self.into_parts();
        match request {
            RenderRequest::Render => {}
            #[allow(unreachable_patterns)]
            _request => return Err(EnvelopeError::Mismatch),
        }
        handle().await;
        response.answer_async(RenderResponse::Render).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum RenderRequest {
    Render,
}

impl RenderRequest {
    pub async fn dispatch_mut<R>(self, render: &mut R) -> RenderResponse
    where
        R: RenderMut + ?Sized,
    {
        match self {
            Self::Render => {
                RenderMut::render(render).await;
                RenderResponse::Render
            }
        }
    }
}

#[derive(Debug)]
pub enum RenderResponse {
    Render,
}

pub struct RenderClient {
    tx: RequestOutbox<RenderRequest, RenderResponse>,
    envelope_size: usize,
}

impl RenderClient {
    pub fn new(envelope_size: usize) -> (Self, RequestInbox<RenderRequest, RenderResponse>) {
        let (tx, rx) = request_channel(32);

        (Self { tx, envelope_size }, rx)
    }
}
#[async_trait::async_trait]
impl Render for RenderClient {
    async fn render(&self) {
        let (envelope, res) = Envelope::new(RenderRequest::Render, self.envelope_size);

        self.tx
            .send_async(envelope)
            .await
            .map_err(|_| TransportError::RequestChannelClosed)
            .unwrap_or_else(|error| panic!("Render::render transport failed: {error}"));

        match res
            .recv_async()
            .await
            .map_err(|_| TransportError::ResponseChannelClosed)
            .unwrap_or_else(|error| panic!("Render::render transport failed: {error}"))
        {
            RenderResponse::Render => (),
            #[allow(unreachable_patterns)]
            _ => panic!(
                "Render::render transport failed: {}",
                TransportError::UnexpectedResponse
            ),
        }
    }
}

// ------- Clock ----------

#[async_trait::async_trait]
pub trait Clock: Send + Sync {
    /// The channel client panics on transport failure. This is the handwritten
    /// equivalent of the future `#[krucyfiks(panic_on_transport)]` policy.
    async fn sleep_ms(&self, ms: u64);
}

#[async_trait::async_trait]
pub trait ClockMut: Send {
    async fn sleep_ms(&mut self, ms: u64);
}

#[async_trait::async_trait]
pub trait ResolveClock: Send {
    type ResolveError;

    async fn resolve_mut<C>(self, clock: &mut C) -> Result<(), Self::ResolveError>
    where
        C: ClockMut + ?Sized;

    async fn resolve_sleep_ms<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(u64) -> Fut + Send,
        Fut: Future<Output = ()> + Send;
}

#[async_trait::async_trait]
impl<E> ResolveClock for AnyRoute<E>
where
    E: EffectTree + Send,
{
    type ResolveError = EnvelopeError<ClockResponse>;

    async fn resolve_mut<C>(self, clock: &mut C) -> Result<(), Self::ResolveError>
    where
        C: ClockMut + ?Sized,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<ClockRequest, ClockResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveClock::resolve_mut(envelope, clock).await
    }

    async fn resolve_sleep_ms<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(u64) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<ClockRequest, ClockResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveClock::resolve_sleep_ms(envelope, handle).await
    }
}

#[async_trait::async_trait]
impl ResolveClock for Envelope<ClockRequest, ClockResponse> {
    type ResolveError = EnvelopeError<ClockResponse>;

    async fn resolve_mut<C>(self, clock: &mut C) -> Result<(), Self::ResolveError>
    where
        C: ClockMut + ?Sized,
    {
        let (request, responder) = self.into_parts();
        let response = request.dispatch_mut(clock).await;
        responder.answer_async(response).await
    }

    async fn resolve_sleep_ms<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(u64) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        let (request, responder) = self.into_parts();
        let ms = match request {
            ClockRequest::SleepMs { ms } => ms,
            #[allow(unreachable_patterns)]
            _ => return Err(EnvelopeError::Mismatch),
        };
        handle(ms).await;
        responder.answer_async(ClockResponse::SleepMs).await
    }
}

#[derive(Debug)]
pub enum ClockRequest {
    SleepMs { ms: u64 },
}

impl ClockRequest {
    pub async fn dispatch_mut<C>(self, clock: &mut C) -> ClockResponse
    where
        C: ClockMut + ?Sized,
    {
        match self {
            Self::SleepMs { ms } => {
                ClockMut::sleep_ms(clock, ms).await;
                ClockResponse::SleepMs
            }
        }
    }
}

#[derive(Debug)]
pub enum ClockResponse {
    SleepMs,
}

pub struct ClockClient {
    tx: RequestOutbox<ClockRequest, ClockResponse>,
    envelope_size: usize,
}

impl ClockClient {
    pub fn new(envelope_size: usize) -> (Self, RequestInbox<ClockRequest, ClockResponse>) {
        let (tx, rx) = request_channel(32);

        (Self { tx, envelope_size }, rx)
    }
}

#[async_trait::async_trait]
impl Clock for ClockClient {
    async fn sleep_ms(&self, ms: u64) {
        let (envelope, res) = Envelope::new(ClockRequest::SleepMs { ms }, self.envelope_size);

        self.tx
            .send_async(envelope)
            .await
            .map_err(|_| TransportError::RequestChannelClosed)
            .unwrap_or_else(|error| panic!("Clock::sleep_ms transport failed: {error}"));

        match res
            .recv_async()
            .await
            .map_err(|_| TransportError::ResponseChannelClosed)
            .unwrap_or_else(|error| panic!("Clock::sleep_ms transport failed: {error}"))
        {
            ClockResponse::SleepMs => (),
            #[allow(unreachable_patterns)]
            _ => panic!(
                "Clock::sleep_ms transport failed: {}",
                TransportError::UnexpectedResponse
            ),
        }
    }
}

// --------- Database --------
#[async_trait::async_trait]
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

#[async_trait::async_trait]
pub trait DatabaseMut: Send {
    async fn count(&mut self) -> Result<Option<usize>, DatabaseError>;

    async fn set_count(&mut self, count: usize) -> Result<(), DatabaseError>;
}

#[async_trait::async_trait]
pub trait ResolveDatabase: Send {
    type ResolveError;

    async fn resolve_mut<D>(self, database: &mut D) -> Result<(), Self::ResolveError>
    where
        D: DatabaseMut + ?Sized;

    async fn resolve_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = Result<Option<usize>, DatabaseError>>;

    async fn resolve_set_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(usize) -> Fut + Send,
        Fut: Send + Future<Output = Result<(), DatabaseError>>;
}

#[async_trait::async_trait]
impl<E> ResolveDatabase for AnyRoute<E>
where
    E: Send + EffectTree,
{
    type ResolveError = EnvelopeError<DatabaseResponse>;

    async fn resolve_mut<D>(self, database: &mut D) -> Result<(), Self::ResolveError>
    where
        D: DatabaseMut + ?Sized,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<DatabaseRequest, DatabaseResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveDatabase::resolve_mut(envelope, database).await
    }

    async fn resolve_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = Result<Option<usize>, DatabaseError>>,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<DatabaseRequest, DatabaseResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveDatabase::resolve_count(envelope, handle).await
    }

    async fn resolve_set_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(usize) -> Fut + Send,
        Fut: Send + Future<Output = Result<(), DatabaseError>>,
    {
        let envelope = self
            .into_inner()
            .extract::<Envelope<DatabaseRequest, DatabaseResponse>>()
            .map_err(|_| EnvelopeError::Mismatch)?;

        ResolveDatabase::resolve_set_count(envelope, handle).await
    }
}

#[async_trait::async_trait]
impl ResolveDatabase for Envelope<DatabaseRequest, DatabaseResponse> {
    type ResolveError = EnvelopeError<DatabaseResponse>;

    async fn resolve_mut<D>(self, database: &mut D) -> Result<(), Self::ResolveError>
    where
        D: DatabaseMut + ?Sized,
    {
        let (request, responder) = self.into_parts();
        let response = request.dispatch_mut(database).await;

        responder.answer_async(response).await
    }

    async fn resolve_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Send + Future<Output = Result<Option<usize>, DatabaseError>>,
    {
        let (request, responser) = self.into_parts();
        match request {
            DatabaseRequest::Count => {}
            _ => return Err(EnvelopeError::Mismatch),
        }
        let response = handle().await;
        responser
            .answer_async(DatabaseResponse::Count(response))
            .await?;
        Ok(())
    }

    async fn resolve_set_count<F, Fut>(self, handle: F) -> Result<(), Self::ResolveError>
    where
        F: FnOnce(usize) -> Fut + Send,
        Fut: Send + Future<Output = Result<(), DatabaseError>>,
    {
        let (request, responser) = self.into_parts();
        let count = match request {
            DatabaseRequest::SetCount { count } => count,
            _ => return Err(EnvelopeError::Mismatch),
        };
        let response = handle(count).await;
        responser
            .answer_async(DatabaseResponse::SetCount(response))
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum DatabaseRequest {
    Count,
    SetCount { count: usize },
}

impl DatabaseRequest {
    pub async fn dispatch_mut<D>(self, database: &mut D) -> DatabaseResponse
    where
        D: DatabaseMut + ?Sized,
    {
        match self {
            Self::Count => DatabaseResponse::Count(DatabaseMut::count(database).await),
            Self::SetCount { count } => {
                DatabaseResponse::SetCount(DatabaseMut::set_count(database, count).await)
            }
        }
    }
}

#[derive(Debug)]
pub enum DatabaseResponse {
    Count(Result<Option<usize>, DatabaseError>),
    SetCount(Result<(), DatabaseError>),
}

type DatabaseEnvelope = Envelope<DatabaseRequest, DatabaseResponse>;
impl From<TransportError> for DatabaseError {
    fn from(error: TransportError) -> Self {
        tracing::error!(?error, "Transport error");
        Self::Transport
    }
}

pub struct DatabaseClient {
    tx: RequestOutbox<DatabaseRequest, DatabaseResponse>,
    envelope_size: usize,
}

impl DatabaseClient {
    pub fn new(envelope_size: usize) -> (Self, RequestInbox<DatabaseRequest, DatabaseResponse>) {
        let (tx, rx) = request_channel(32);

        (Self { tx, envelope_size }, rx)
    }
}

#[async_trait::async_trait]
impl Database for DatabaseClient {
    async fn count(&self) -> Result<Option<usize>, DatabaseError> {
        let (envelope, response) =
            DatabaseEnvelope::new(DatabaseRequest::Count, self.envelope_size);
        self.tx
            .send_async(envelope)
            .await
            .map_err(|_| TransportError::RequestChannelClosed)?;

        match response
            .recv_async()
            .await
            .map_err(|_| TransportError::ResponseChannelClosed)?
        {
            DatabaseResponse::Count(c) => c,
            _ => Err(TransportError::UnexpectedResponse)?,
        }
    }

    async fn set_count(&self, count: usize) -> Result<(), DatabaseError> {
        let (envelope, response) =
            DatabaseEnvelope::new(DatabaseRequest::SetCount { count }, self.envelope_size);
        self.tx
            .send_async(envelope)
            .await
            .map_err(|_| TransportError::RequestChannelClosed)?;

        match response
            .recv_async()
            .await
            .map_err(|_| TransportError::ResponseChannelClosed)?
        {
            DatabaseResponse::SetCount(c) => c,
            _ => Err(TransportError::UnexpectedResponse)?,
        }
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
#[derive(Debug)]
pub enum CounterEffect {
    Render(Envelope<RenderRequest, RenderResponse>),
    Clock(Envelope<ClockRequest, ClockResponse>),
    Database(Envelope<DatabaseRequest, DatabaseResponse>),
}

impl EffectTree for CounterEffect {
    fn extract<T: 'static>(self) -> Result<T, Self> {
        match self {
            Self::Render(envelope) => envelope.extract::<T>().map_err(Self::Render),
            Self::Clock(envelope) => envelope.extract::<T>().map_err(Self::Clock),
            Self::Database(envelope) => envelope.extract::<T>().map_err(Self::Database),
        }
    }
}

impl CounterEffect {
    pub fn try_into_render(self) -> Result<Envelope<RenderRequest, RenderResponse>, Self> {
        match self {
            CounterEffect::Render(envelope) => Ok(envelope),
            _ => Err(self),
        }
    }
    #[track_caller]
    pub fn expect_render(self) -> Envelope<RenderRequest, RenderResponse> {
        match self {
            Self::Render(envelope) => envelope,
            Self::Clock(_) => panic!("expected CounterEffect::Render, got CounterEffect::Clock"),
            Self::Database(_) => {
                panic!("expected CounterEffect::Render, got CounterEffect::Database")
            }
        }
    }

    pub fn try_into_clock(self) -> Result<Envelope<ClockRequest, ClockResponse>, Self> {
        match self {
            CounterEffect::Clock(envelope) => Ok(envelope),
            _ => Err(self),
        }
    }
    #[track_caller]
    pub fn expect_clock(self) -> Envelope<ClockRequest, ClockResponse> {
        match self {
            Self::Clock(envelope) => envelope,
            Self::Render(_) => panic!("expected CounterEffect::Clock, got CounterEffect::Render"),
            Self::Database(_) => {
                panic!("expected CounterEffect::Clock, got CounterEffect::Database")
            }
        }
    }

    pub fn try_into_database(self) -> Result<Envelope<DatabaseRequest, DatabaseResponse>, Self> {
        match self {
            CounterEffect::Database(envelope) => Ok(envelope),
            _ => Err(self),
        }
    }

    #[track_caller]
    pub fn expect_database(self) -> Envelope<DatabaseRequest, DatabaseResponse> {
        match self {
            Self::Database(envelope) => envelope,
            Self::Render(_) => {
                panic!("expected CounterEffect::Database, got CounterEffect::Render")
            }
            Self::Clock(_) => panic!("expected CounterEffect::Database, got CounterEffect::Clock"),
        }
    }
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
        let (render, render_inbox) = RenderClient::new(1);
        let (clock, clock_inbox) = ClockClient::new(1);
        let (db, database_inbox) = DatabaseClient::new(1);

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
