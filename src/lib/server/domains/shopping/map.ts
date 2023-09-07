import type { GroupedRecipeIngredient } from '$shared/domains/recipe/models';
import type { ShoppingListItem, StoredShoppingListItem } from '$shared/domains/shopping/models';
import type {
	CreateShoppingListItem,
	UpdateShoppingListItem
} from '$shared/domains/shopping/schema';
import type { Prisma } from '@prisma/client';
import { v4 } from 'uuid';
import type { RecipeIngredientMap } from '../recipe/map';

export class ShoppingListItemMap {
	constructor(readonly ingredientMap: RecipeIngredientMap) {}

	toCreateInputFromRecipeIngerdient(
		ingredient: GroupedRecipeIngredient
	): Prisma.ShoppingListItemCreateInput {
		return {
			id: v4(),
			name: ingredient.raw?.name ?? ingredient.name,
			amount: ingredient.amount.toNumber(),
			unit: ingredient.unit,
			state: 'TODO',
			ingredients: {
				connect: ingredient.ids.map((id) => ({ id }))
			}
		};
	}

	toUpdateInputFromRecipeIngerdient(
		ingredient: GroupedRecipeIngredient,
		clear: boolean
	): Prisma.ShoppingListItemUpdateInput {
		return {
			name: ingredient.raw?.name ?? ingredient.name,
			amount: clear
				? ingredient.amount.toNumber()
				: {
						increment: ingredient.amount.toNumber()
				  },
			unit: ingredient.unit,
			state: 'TODO',
			ingredients: clear
				? {
						set: ingredient.ids.map((id) => ({ id }))
				  }
				: {
						connect: ingredient.ids.map((id) => ({ id }))
				  }
		};
	}

	toDomain(item: StoredShoppingListItem): ShoppingListItem {
		return {
			id: item.id,
			name: item.name,
			amount: item.amount.toNumber(),
			unit: item.unit,
			order: item.order,
			state: item.state,
			ingredients: item.ingredients.map((i) => this.ingredientMap.toDomainWithRecipe(i))
		};
	}

	toCreateInput(item: CreateShoppingListItem): Prisma.ShoppingListItemCreateInput {
		return {
			id: item.id,
			name: item.name,
			state: item.state,
			amount: item.amount,
			unit: item.unit ?? null
		};
	}

	toUpdateInput(item: UpdateShoppingListItem): Prisma.ShoppingListItemUpdateInput {
		return {
			state: item.state,
			amount: item.amount,
			unit: item.unit
		};
	}
}
