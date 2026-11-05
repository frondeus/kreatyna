//! Recipes: the hand-made recipe of the Recipe MVP epic (D-06).

pub mod capabilities;
mod detail;
mod duration;
mod editor;
mod graph;
mod ingredient_lines;
mod list;
mod model;
mod section;
mod step_canvas;

pub use detail::{
    RecipeDeletion, RecipeDetail, RecipeDetailEvent, RecipeDetailLine, RecipeDetailOutcome,
    RecipeDetailStatus, RecipeDetailStep, RecipeDetailView,
};
pub use duration::{DurationChip, DurationEntry, DurationError, DurationInput, DurationParts};
pub use editor::{
    RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem, RecipeEditorStatus,
    RecipeEditorView,
};
pub use graph::{ConnectRefusal, StepEdge, StepEdges};
pub use ingredient_lines::{IngredientLineView, IngredientLinesEvent};
pub use list::{
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus, RecipeListView,
};
pub use model::{
    CanvasPoint, IngredientLine, IngredientLineId, IngredientLineIdParseError, IngredientLineName,
    IngredientLineNameError, KnownUnit, NewRecipe, Recipe, RecipeId, RecipeIdParseError,
    RecipeIngredientAmount, RecipeIngredientAmountError, RecipeIngredientNote,
    RecipeIngredientNoteError, RecipeIngredientQuantity, RecipeSummary, RecipeTitle,
    RecipeTitleError, RecipeUpdate, Step, StepDurationS, StepDurationSError, StepId,
    StepIdParseError, StepInstructions, StepInstructionsError, StepKind, StepTitle, StepTitleError,
    StepWrite,
};
pub use section::{
    RecipePaneOutcome, RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome,
    RecipeSectionView,
};
pub use step_canvas::{
    StepAfterView, StepCanvasEvent, StepCanvasView, StepConnectionView, StepDurationView,
    StepLabel, StepNodeView, StepNodesView, StepProblems,
};
