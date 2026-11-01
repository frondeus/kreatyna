//! Recipes: the hand-made recipe of the Recipe MVP epic (D-06).

pub mod capabilities;
mod list;
mod model;

pub use list::{
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus, RecipeListView,
};
pub use model::{NewRecipe, Recipe, RecipeId, RecipeTitle, RecipeTitleError};
