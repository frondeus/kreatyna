//! Recipes: the hand-made recipe of the Recipe MVP epic (D-06).

pub mod capabilities;
mod editor;
mod list;
mod model;
mod section;

pub use editor::{
    RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem, RecipeEditorStatus,
    RecipeEditorView,
};
pub use list::{
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus, RecipeListView,
};
pub use model::{NewRecipe, Recipe, RecipeId, RecipeTitle, RecipeTitleError};
pub use section::{
    RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome, RecipeSectionView,
};
