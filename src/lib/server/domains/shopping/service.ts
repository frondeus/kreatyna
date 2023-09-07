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
		await Promise.all(
			ingredients.map((i) => this.updateOrCreateShoppingListItemLinkedToIngredient(i))
		);
	}

	private async updateOrCreateShoppingListItemLinkedToIngredient(
		ingredient: GroupedRecipeIngredient
	): Promise<void> {
		const existing = await this.prisma.shoppingListItem.findFirst({
			...selectShoppingListItems,
			where: {
				ingredients: {
					some: {
						raw_id: ingredient.raw_id,
						unit: ingredient.unit
					}
				}
			}
		});

		if (existing) {
			await this.prisma.shoppingListItem.update({
				data: this.itemMap.toUpdateInputFromRecipeIngerdient(
					ingredient,
					existing.state === 'BOUGHT'
				),
				where: {
					id: existing.id
				}
			});
			return;
		}

		await this.prisma.shoppingListItem.create({
			data: this.itemMap.toCreateInputFromRecipeIngerdient(ingredient)
		});
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
		await this.prisma.shoppingListItem.updateMany({
			data: {
				state: 'BOUGHT'
			},
			where: {
				state: 'IN_CART'
			}
		});
	}
}
