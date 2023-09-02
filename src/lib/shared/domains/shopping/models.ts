import type { Prisma } from '@prisma/client';
import type { selectShoppingListItems } from './queries';

export type ShoppingListItem = Prisma.ShoppingListItemGetPayload<typeof selectShoppingListItems>;
