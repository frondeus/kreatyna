import type { PrismaClient } from '@prisma/client';
import type { ShoppingListItemMap } from './map';
import type { ShoppingListItem } from '$shared/domains/shopping/models';
import { selectShoppingListItems } from '$shared/domains/shopping/queries';
import { sortByState } from '$shared/domains/shopping/utils';
import type {
	CreateShoppingListItem,
	UpdateShoppingListItem
} from '$shared/domains/shopping/schema';
import type { GroupedRecipeIngredient } from '$shared/domains/recipe/models';

export class ShoppingListService {
	constructor(readonly prisma: PrismaClient, readonly itemMap: ShoppingListItemMap) {}

	async getShoppingListItems(): Promise<ShoppingListItem[]> {
		const items = await this.prisma.shoppingListItem.findMany({
			...selectShoppingListItems
		});
		return items.map((item) => this.itemMap.toDomain(item)).sort(sortByState);
	}

	async getShoppingListItem(id: string): Promise<ShoppingListItem | null> {
		const item = await this.prisma.shoppingListItem.findUnique({
			...selectShoppingListItems,
			where: {
				id
			}
		});

		return item ? this.itemMap.toDomain(item) : null;
	}

	async updateShoppingListItem(update: UpdateShoppingListItem): Promise<void> {
		await this.prisma.shoppingListItem.update({
			data: this.itemMap.toUpdateInput(update),
			where: {
				id: update.id
			}
		});
	}

	async createShoppingListItemFromRecipeIngredients(
		ingredients: GroupedRecipeIngredient[]
	): Promise<void> {
		const items = ingredients.map((i) => this.itemMap.toCreateInputFromRecipeIngerdient(i));

		console.log({ items });

		for (const item of items) {
			await this.prisma.shoppingListItem.create({
				data: item
			});
		}
	}

	async upsertShoppingListItem(update: CreateShoppingListItem): Promise<void> {
		await this.prisma.shoppingListItem.upsert({
			create: {
				...this.itemMap.toCreateInput(update)
			},
			update: {
				...this.itemMap.toUpdateInput(update)
			},
			where: {
				id: update.id
			}
		});
	}

	async pruneShoppingList(): Promise<void> {
		await this.prisma.shoppingListItem.deleteMany({});
	}
}
