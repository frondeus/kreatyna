import type { RecipeCreateOrUpdate } from '$lib/shared/domains/recipe/models';
import type { RecipeResponse, Ingredient } from './models';

export class AIMap {
	public toExample(recipe: RecipeCreateOrUpdate): RecipeResponse {
		return {
			name: recipe.name ?? '',
			steps: recipe.steps,
			ingredients: recipe.ingredients.map((i) => i as Ingredient),
			portions: recipe.portions
		};
	}
}
