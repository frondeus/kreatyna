import type { Prisma } from '@prisma/client';
import type { selectShoppingListItems } from './queries';
import type { ShoppingListState } from '@prisma/client';

export type StoredShoppingListItem = Prisma.ShoppingListItemGetPayload<
	typeof selectShoppingListItems
>;
export type ShoppingListItem = Omit<StoredShoppingListItem, 'amount'> & { amount: number };

export type { ShoppingListState };
