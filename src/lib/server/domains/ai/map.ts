import type {
	GroupedRecipeIngredient,
	RecipeCreate,
	RecipeCreateOrUpdate
} from '$lib/shared/domains/recipe/models';
import type { ShoppingListItem, StoredShoppingListItem } from '$shared/domains/shopping/models';
import type { Category } from '$shared/domains/shopping/schema';
import type { CategoryPrompt, CategoryResponse } from './models/category';
import type { RecipeResponse, Ingredient } from './models/recipe';

export class AIMap {
	public toRecipeExample(recipe: RecipeCreateOrUpdate): RecipeResponse {
		return {
			name: recipe.name ?? '',
			steps: recipe.steps,
			ingredients: recipe.ingredients.map((i) => i as Ingredient),
			portions: recipe.portions
		};
	}

	public toRecipe(model: RecipeResponse): RecipeCreate {
		return {
			name: model.name,
			steps: model.steps,
			portions: model.portions,
			ingredients: model.ingredients.map((i) => ({
				name: i.name,
				amount: i.amount,
				unit: i.unit
			})),
			tags: []
		};
	}

	public toCategoryResponse(
		item: ShoppingListItem | StoredShoppingListItem | null
	): CategoryResponse | null {
		if (item?.category === null || item?.category === undefined) {
			return null;
		}

		return {
			categoryId: item.category.id,
			categoryName: item.category.name,
			categoryEmoji: item.category.icon
		};
	}

	public toCategory(response: CategoryResponse): Category {
		return {
			id: response.categoryId,
			name: response.categoryName,
			emoji: response.categoryEmoji
		};
	}

	public toCategoryPromptFromGroup(item: GroupedRecipeIngredient): CategoryPrompt {
		return {
			name: item.name,
			ingredients:
				item.raw === null
					? []
					: [
							{
								name: item.raw.name,
								recipeName: '',
								recipeTags: [],
								rawName: item.raw.name
							}
					  ]
		};
	}

	public toCategoryPrompt(item: ShoppingListItem): CategoryPrompt {
		return {
			name: item.name,
			ingredients: item.ingredients.map((i) => ({
				name: i.name,
				recipeName: i.recipe.name,
				recipeTags: i.recipe.tags.map((t) => `${t.category}:${t.name}`),
				rawName: i.raw?.name ?? null
			}))
		};
	}
}
