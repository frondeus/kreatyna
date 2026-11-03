use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::effects::{Envelope, TransportError};

use super::{NewRecipe, Recipe, RecipeId, RecipeUpdate};
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

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecipeSaveError {
    #[error("Recipe title already exists")]
    DuplicateTitle,
    /// Only `update`: no recipe has this id any more.
    #[error("Recipe to update was not found")]
    NotFound,
    #[error("Recipe storage failed")]
    Storage,
    #[error("Recipe store transport failed")]
    Transport,
}

impl From<TransportError> for RecipeSaveError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecipeDeleteError {
    #[error("Recipe storage failed")]
    Storage,
    #[error("Recipe store transport failed")]
    Transport,
}

impl From<TransportError> for RecipeDeleteError {
    fn from(_: TransportError) -> Self {
        Self::Transport
    }
}

/// Persistence boundary for recipes. Its implementation owns identity, title
/// uniqueness, and idempotent deletion.
#[krucyfiks::capability]
pub trait RecipeStore: Send + Sync {
    /// All recipes, sorted by title key (`ingredients::name_key` of the
    /// title). Title keys are unique, so the order is complete.
    async fn list(&self) -> Result<Vec<Recipe>, RecipeLoadError>;
    /// The recipe with this id, or `None` when no recipe has it.
    async fn get(&self, id: RecipeId) -> Result<Option<Recipe>, RecipeLoadError>;
    /// Saves a new recipe with a new id. Fails with `DuplicateTitle` when
    /// another recipe has the same title key.
    async fn create(&self, recipe: NewRecipe) -> Result<Recipe, RecipeSaveError>;
    /// Saves new data for the recipe with this id. Fails with `NotFound` when
    /// no recipe has the id (it never inserts one), and with `DuplicateTitle`
    /// when another recipe has the same title key.
    async fn update(&self, id: RecipeId, recipe: RecipeUpdate) -> Result<Recipe, RecipeSaveError>;
    /// Deletes the recipe with this id and every row that it owns. Deleting
    /// a recipe that does not exist succeeds (idempotent).
    async fn delete(&self, id: RecipeId) -> Result<(), RecipeDeleteError>;
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
