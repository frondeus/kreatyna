import type { PrismaClient } from '@prisma/client';
import type { ShoppingListItemMap } from './map';
import type { ShoppingListItem } from '$shared/domains/shopping/models';
import { selectShoppingListItems } from '$shared/domains/shopping/queries';
import { sortByState } from '$shared/domains/shopping/utils';

export class ShoppingListService {
	constructor(readonly prisma: PrismaClient, readonly itemMap: ShoppingListItemMap) {}

	async getShoppingListItems(): Promise<ShoppingListItem[]> {
		const items = await this.prisma.shoppingListItem.findMany({
			...selectShoppingListItems
		});
		return items.map((item) => this.itemMap.toDomain(item)).sort(sortByState);
	}
}
