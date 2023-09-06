import type { Prisma } from '@prisma/client';
import type { selectShoppingListItems } from './queries';
import type { ShoppingListState } from '@prisma/client';
import type { Ingredient } from '../recipe/models';

export type StoredShoppingListItem = Prisma.ShoppingListItemGetPayload<
	typeof selectShoppingListItems
>;
export type ShoppingListItem = Omit<StoredShoppingListItem, 'amount' | 'ingredients'> & {
	amount: number;
	ingredients: Ingredient[];
};

export type { ShoppingListState };
