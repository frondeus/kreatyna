//! Recipes: the hand-made recipe of the Recipe MVP epic (D-06).

pub mod capabilities;
mod detail;
mod editor;
mod list;
mod model;
mod section;

pub use detail::{
    RecipeDeletion, RecipeDetail, RecipeDetailEvent, RecipeDetailOutcome, RecipeDetailStatus,
    RecipeDetailView,
};
pub use editor::{
    RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem, RecipeEditorStatus,
    RecipeEditorView,
};
pub use list::{
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus, RecipeListView,
};
pub use model::{
    CanvasPoint, NewRecipe, Recipe, RecipeId, RecipeIdParseError, RecipeSummary, RecipeTitle,
    RecipeTitleError, RecipeUpdate, Step, StepId, StepIdParseError, StepTitle, StepTitleError,
    StepWrite,
};
pub use section::{
    RecipePaneOutcome, RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome,
    RecipeSectionView,
};
