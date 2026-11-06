//! Recipes: the hand-made recipe of the Recipe MVP epic (D-06).

pub mod capabilities;
mod detail;
mod duration;
mod editor;
mod graph;
mod list;
mod model;
mod recipe_ingredients;
mod section;
mod step_canvas;

pub use detail::{
    RecipeDeletion, RecipeDetail, RecipeDetailEvent, RecipeDetailIngredient, RecipeDetailOutcome,
    RecipeDetailStatus, RecipeDetailStep, RecipeDetailView,
};
pub use duration::{DurationChip, DurationEntry, DurationError, DurationInput, DurationParts};
pub use editor::{
    RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem, RecipeEditorStatus,
    RecipeEditorView,
};
pub use graph::{ConnectRefusal, StepEdge, StepEdges};
pub use list::{
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus, RecipeListView,
};
pub use model::{
    CanvasPoint, KnownUnit, NewRecipe, Recipe, RecipeId, RecipeIdParseError, RecipeIngredient,
    RecipeIngredientAmount, RecipeIngredientAmountError, RecipeIngredientId,
    RecipeIngredientIdParseError, RecipeIngredientName, RecipeIngredientNameError,
    RecipeIngredientNote, RecipeIngredientNoteError, RecipeIngredientQuantity, RecipeSummary,
    RecipeTitle, RecipeTitleError, RecipeUpdate, Step, StepDurationS, StepDurationSError, StepId,
    StepIdParseError, StepInstructions, StepInstructionsError, StepKind, StepTitle, StepTitleError,
    StepWrite,
};
pub use recipe_ingredients::{RecipeIngredientView, RecipeIngredientsEvent};
pub use section::{
    RecipePaneOutcome, RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome,
    RecipeSectionView,
};
pub use step_canvas::{
    StepAfterView, StepCanvasEvent, StepCanvasView, StepConnectionView, StepDurationView,
    StepLabel, StepNodeView, StepNodesView, StepProblems,
};
