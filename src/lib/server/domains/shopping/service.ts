import type { PrismaClient, ShoppingListState } from '@prisma/client';
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

	async getShoppingListItem(id: number): Promise<ShoppingListItem | null> {
		const item = await this.prisma.shoppingListItem.findUnique({
			...selectShoppingListItems,
			where: {
				id
			}
		});

		return item ? this.itemMap.toDomain(item) : null;
	}

	async updateShoppingListItemState(id: number, state: ShoppingListState): Promise<void> {
		await this.prisma.shoppingListItem.update({
			data: {
				state
			},
			where: {
				id
			}
		});
	}
}
