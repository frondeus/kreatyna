use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{Envelope, TransportError};

use super::{Ingredient, IngredientId, IngredientWrite};
use crate::capabilities::{Render, RenderClient, RenderRequest, RenderResponse};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IngredientLoadError {
    #[error("Ingredient storage failed")]
    Storage,
    #[error("Ingredient store transport failed")]
    Transport,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IngredientSaveError {
    #[error("Ingredient name already exists")]
    DuplicateName,
    #[error("Ingredient to update was not found")]
    NotFound,
    #[error("Ingredient storage failed")]
    Storage,
    #[error("Ingredient store transport failed")]
    Transport,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IngredientDeleteError {
    #[error("Ingredient storage failed")]
    Storage,
    #[error("Ingredient store transport failed")]
    Transport,
}

impl From<TransportError> for IngredientLoadError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

impl From<TransportError> for IngredientSaveError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

impl From<TransportError> for IngredientDeleteError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

/// Persistence boundary. Its implementation owns identity assignment, uniqueness,
/// update existence, exact value round trips, and idempotent deletion.
#[krucyfiks::capability]
pub trait IngredientStore: Send + Sync {
    async fn list(&self) -> Result<Vec<Ingredient>, IngredientLoadError>;
    async fn save(&self, write: IngredientWrite) -> Result<Ingredient, IngredientSaveError>;
    async fn delete(&self, id: IngredientId) -> Result<(), IngredientDeleteError>;
}

#[derive(Clone)]
pub struct IngredientCapabilities {
    pub render: Arc<dyn Render>,
    pub store: Arc<dyn IngredientStore>,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum IngredientEffect {
    Render(Envelope<RenderRequest, RenderResponse>),
    Store(Envelope<IngredientStoreRequest, IngredientStoreResponse>),
}

pub type IngredientEffectStream = BoxStream<'static, IngredientEffect>;

impl IngredientCapabilities {
    pub fn testing() -> (Self, IngredientEffectStream) {
        let (render, render_inbox) = RenderClient::new(32, 1);
        let (store, store_inbox) = IngredientStoreClient::new(32, 1);
        let streams = vec![
            render_inbox
                .into_stream()
                .map(IngredientEffect::Render)
                .boxed(),
            store_inbox
                .into_stream()
                .map(IngredientEffect::Store)
                .boxed(),
        ];
        (
            Self {
                render: Arc::new(render),
                store: Arc::new(store),
            },
            select_all(streams).boxed(),
        )
    }
}
