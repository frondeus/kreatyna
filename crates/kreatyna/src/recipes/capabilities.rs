use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{Envelope, TransportError};

use super::Recipe;
use crate::{
    CLIENT_CAPABILITY,
    capabilities::{Render, RenderClient, RenderRequest},
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecipeLoadError {
    #[error("Recipe storage failed")]
    Storage,
    #[error("Recipe store transport failed")]
    Transport,
}

impl From<TransportError> for RecipeLoadError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

/// Persistence boundary for recipes. Its implementation owns identity and
/// the title rule; REC-02 adds `save`.
#[krucyfiks::capability]
pub trait RecipeStore: Send + Sync {
    /// All recipes, sorted by title key (`ingredients::name_key` of the
    /// title). Title keys are unique, so the order is complete.
    async fn list(&self) -> Result<Vec<Recipe>, RecipeLoadError>;
}

#[derive(Clone)]
pub struct RecipeCapabilities {
    pub render: Arc<dyn Render>,
    pub store: Arc<dyn RecipeStore>,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum RecipeEffect {
    Render(Envelope<RenderRequest>),
    Store(Envelope<RecipeStoreRequest>),
}

pub type RecipeEffectStream = BoxStream<'static, RecipeEffect>;

impl RecipeCapabilities {
    pub fn testing() -> (Self, RecipeEffectStream) {
        let (render, render_inbox) = RenderClient::new(CLIENT_CAPABILITY);
        let (store, store_inbox) = RecipeStoreClient::new(CLIENT_CAPABILITY);
        let streams = vec![
            render_inbox.into_stream().map(RecipeEffect::Render).boxed(),
            store_inbox.into_stream().map(RecipeEffect::Store).boxed(),
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
