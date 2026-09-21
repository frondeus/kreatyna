use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{Envelope, EnvelopeError, RequestInbox, RequestOutbox, request_channel};

#[async_trait::async_trait]
pub trait Render: Send + Sync {
    async fn render(&self);
}

//--- in future derived
#[async_trait::async_trait]
pub trait RenderMut: Send + Sync {
    async fn render(&mut self);
}

#[async_trait::async_trait]
pub trait ResolveRender: Send + Sync {
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

    async fn resolve_render<F, Fut>(mut self, handle: F) -> Result<(), Self::ResolveError>
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
        let _response = handle().await;
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

        self.tx.send_async(envelope).await.unwrap();

        match res.recv_async().await.unwrap() {
            RenderResponse::Render => (),
            #[allow(unreachable_patterns)]
            _ => panic!("Expected render capability"),
        }
    }
}

// ------- Clock ----------

#[async_trait::async_trait]
pub trait Clock: Send + Sync {
    async fn sleep_ms(&self, ms: u64);
}

#[derive(Debug)]
pub enum ClockRequest {
    SleepMs { ms: u64 },
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

        self.tx.send_async(envelope).await.unwrap();

        match res.recv_async().await.unwrap() {
            ClockResponse::SleepMs => (),
            #[allow(unreachable_patterns)]
            _ => panic!("Expected clock capability"),
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
pub trait ResolveDatabase: Send + Sync {
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
type DatabaseEnvelopeError = EnvelopeError<DatabaseEnvelope>;
impl<Req, Res> From<EnvelopeError<Envelope<Req, Res>>> for DatabaseError
where
    EnvelopeError<Envelope<Req, Res>>: std::fmt::Debug,
{
    fn from(error: EnvelopeError<Envelope<Req, Res>>) -> Self {
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
            .map_err(DatabaseEnvelopeError::from)?;

        match response
            .recv_async()
            .await
            .map_err(DatabaseEnvelopeError::from)?
        {
            DatabaseResponse::Count(c) => c,
            _ => Err(DatabaseEnvelopeError::Mismatch)?,
        }
    }

    async fn set_count(&self, count: usize) -> Result<(), DatabaseError> {
        let (envelope, response) =
            DatabaseEnvelope::new(DatabaseRequest::SetCount { count }, self.envelope_size);
        self.tx
            .send_async(envelope)
            .await
            .map_err(DatabaseEnvelopeError::from)?;

        match response
            .recv_async()
            .await
            .map_err(DatabaseEnvelopeError::from)?
        {
            DatabaseResponse::SetCount(c) => c,
            _ => Err(DatabaseEnvelopeError::Mismatch)?,
        }
    }
}

// ---- App

#[derive(Debug)]
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
