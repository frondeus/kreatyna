import type { TagWithCategory } from '$shared/domains/tags/models';
import type { ChosenRecipe } from './populator';

export class ExistingRecipeSelector {
	selectChosenRecipe(recipe: ChosenRecipe, tags: TagWithCategory[], forDay: number): boolean {
		const recipeTags = recipe.recipe.tags.map((tag) => tag.id);
		if (recipe.used >= 2) {
			return false;
		}
		if (!tags.every((tag) => recipeTags.includes(tag.id))) {
			return false;
		}
		const dayDist = Math.abs(recipe.day - forDay);
		if (dayDist >= 2) {
			return false;
		}
		return true;
	}
}
