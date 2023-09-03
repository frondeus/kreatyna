import type { PrismaClient } from '@prisma/client';
import type { ShoppingListItemMap } from './map';
import type { ShoppingListItem } from '$shared/domains/shopping/models';
import { selectShoppingListItems } from '$shared/domains/shopping/queries';

const sortByState: (a: ShoppingListItem, b: ShoppingListItem) => number = (a, b) => {
	if (a.state !== b.state) {
		if (a.state === 'TODO') return -1;
		if (b.state === 'TODO') return 1;

		if (a.state === 'IN_CART') return -1;
		if (b.state === 'IN_CART') return 1;
	}

	return a.order - b.order;
};

export class ShoppingListService {
	constructor(readonly prisma: PrismaClient, readonly itemMap: ShoppingListItemMap) {}

	async getShoppingListItems(): Promise<ShoppingListItem[]> {
		const items = await this.prisma.shoppingListItem.findMany({
			...selectShoppingListItems
		});
		return items.map((item) => this.itemMap.toDomain(item)).sort(sortByState);
	}
}
