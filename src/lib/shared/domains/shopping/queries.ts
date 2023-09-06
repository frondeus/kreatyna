import { Prisma } from '@prisma/client';
import { ingredients } from '../recipe/queries';

export const selectShoppingListItems = Prisma.validator<Prisma.ShoppingListItemFindManyArgs>()({
	select: {
		id: true,
		name: true,
		amount: true,
		unit: true,
		state: true,
		order: true,
		ingredients: ingredients
	}
});
