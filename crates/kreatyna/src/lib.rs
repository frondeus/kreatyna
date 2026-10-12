pub mod app;
pub use app::{
    AppCapabilities, AppEffect, AppEffectStream, AppEvent, AppLayout, AppOutcome, AppView,
    AppWidget, IngredientsOutcome, IngredientsView,
};

pub mod capabilities;

pub mod confirmation;
pub mod ingredients;
mod scoped_child;

pub const CLIENT_CAPABILITY: usize = 32;
