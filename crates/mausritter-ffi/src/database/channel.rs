use flume::{Receiver, RecvError, SendError, Sender};
use mausritter::capabilities::{Database, DatabaseError};

#[derive(Debug, thiserror::Error)]
pub enum EnvelopeError<Envelope> {
    #[error(transparent)]
    Request(#[from] SendError<Envelope>),
    #[error(transparent)]
    Response(#[from] RecvError),
    #[error("There is a mismatch between request and response types")]
    Mismatch,
}

impl From<DatabaseEnvelopeError> for DatabaseError {
    fn from(error: DatabaseEnvelopeError) -> Self {
        tracing::error!(?error, "Transport error");
        Self::Transport
    }
}

pub type DatabaseEnvelope = Envelope<DatabaseRequest, DatabaseResponse>;
pub type DatabaseEnvelopeError = EnvelopeError<DatabaseEnvelope>;

pub struct DatabaseClient {
    tx: Sender<DatabaseEnvelope>,
}

impl DatabaseClient {
    pub fn new() -> (Self, Receiver<DatabaseEnvelope>) {
        let (tx, rx) = flume::bounded(32);

        (Self { tx }, rx)
    }
}

#[async_trait::async_trait]
impl Database for DatabaseClient {
    async fn count(&self) -> Result<Option<usize>, DatabaseError> {
        let (envelope, response) = DatabaseEnvelope::new(DatabaseRequest::Count);
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
        let (envelope, response) = DatabaseEnvelope::new(DatabaseRequest::SetCount { count });
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

#[derive(Debug)]
pub struct Envelope<Request, Response> {
    pub request: Request,
    pub response: Sender<Response>,
}

impl<Request, Response> Envelope<Request, Response> {
    pub fn new(request: Request) -> (Self, Receiver<Response>) {
        let (response, receiver) = flume::bounded(0);
        (Self { request, response }, receiver)
    }
}

#[derive(Debug)]
pub enum DatabaseRequest {
    Count,
    SetCount { count: usize },
}

#[derive(Debug)]
pub enum DatabaseResponse {
    Count(Result<Option<usize>, DatabaseError>),
    SetCount(Result<(), DatabaseError>),
}
