import type { RecipeIngredient } from '$shared/domains/recipe/models';
import type { ShoppingListItem, StoredShoppingListItem } from '$shared/domains/shopping/models';
import type {
	CreateShoppingListItem,
	UpdateShoppingListItem
} from '$shared/domains/shopping/schema';
import type { Prisma } from '@prisma/client';
import { v4 } from 'uuid';

export class ShoppingListItemMap {
	toCreateInputFromRecipeIngerdient(ingredient: RecipeIngredient): CreateShoppingListItem {
		return {
			id: v4(),
			name: ingredient.raw?.name ?? ingredient.name,
			amount: ingredient.amount.toNumber(),
			unit: ingredient.unit ?? undefined,
			state: 'TODO'
		};
	}

	toDomain(item: StoredShoppingListItem): ShoppingListItem {
		return {
			id: item.id,
			name: item.name,
			amount: item.amount.toNumber(),
			unit: item.unit,
			order: item.order,
			state: item.state
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
