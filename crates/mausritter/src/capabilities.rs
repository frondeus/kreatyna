#[async_trait::async_trait]
pub trait Render: Send + Sync {
    async fn render(&self);
}

#[async_trait::async_trait]
pub trait Clock: Send + Sync {
    async fn sleep_ms(&self, ms: u64);
}

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
