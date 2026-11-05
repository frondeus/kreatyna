pub mod app;
pub use app::{
    AppCapabilities, AppEffect, AppEffectStream, AppEvent, AppLayout, AppOutcome, AppSection,
    AppView, AppWidget, SplitAppEvent, SplitAppView,
};

pub mod capabilities;

pub mod confirmation;
pub mod ingredients;
mod problem_display;
pub mod recipes;
mod scoped_child;

pub const CLIENT_CAPABILITY: usize = 32;
