import type { Prisma } from '@prisma/client';
import type { selectShoppingListCategoryForItem, selectShoppingListItems } from './queries';
import type { ShoppingListState } from '@prisma/client';
import type { IngredientWithRecipe } from '../recipe/models';

export type StoredShoppingListItem = Prisma.ShoppingListItemGetPayload<
	typeof selectShoppingListItems
>;
export type ShoppingListItem = Omit<StoredShoppingListItem, 'amount' | 'ingredients'> & {
	amount: number;
	ingredients: IngredientWithRecipe[];
};

export type StoredShoppingListCategoryForItem = Prisma.ShoppingListCategoryGetPayload<
	typeof selectShoppingListCategoryForItem
>;

export type ShoppingListCategoryForItem = StoredShoppingListCategoryForItem;

export type { ShoppingListState };
