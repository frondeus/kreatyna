import type { PrismaClient } from '@prisma/client';
import type { ShoppingListItemMap } from './map';
import type {
	ShoppingListCategoryForItem,
	ShoppingListItem
} from '$shared/domains/shopping/models';
import {
	selectShoppingListCategoryForItem,
	selectShoppingListItems
} from '$shared/domains/shopping/queries';
import { sortByState } from '$shared/domains/shopping/utils';
import type {
	CreateShoppingListItemWithCategory,
	UpdateShoppingListItemWithCategory
} from '$shared/domains/shopping/schema';
import type { GroupedRecipeIngredient } from '$shared/domains/recipe/models';
import type { AIService } from '../ai/service';
import type { AIMap } from '../ai/map';

export class ShoppingListService {
	constructor(
		readonly prisma: PrismaClient,
		readonly itemMap: ShoppingListItemMap,
		readonly ai: AIService,
		readonly aiMap: AIMap
	) {}

	async getShoppingListItems(): Promise<ShoppingListItem[]> {
		const items = await this.prisma.shoppingListItem.findMany({
			...selectShoppingListItems,
			orderBy: {
				category: {
					order: 'asc'
				}
			}
		});
		return items.map((item) => this.itemMap.toDomain(item)).sort(sortByState);
	}

	async getShoppingListCategories(): Promise<ShoppingListCategoryForItem[]> {
		return await this.prisma.shoppingListCategory.findMany({
			...selectShoppingListCategoryForItem,
			orderBy: {
				order: 'asc'
			}
		});
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

	async updateShoppingListItem(update: UpdateShoppingListItemWithCategory): Promise<void> {
		await this.prisma.shoppingListItem.update({
			data: this.itemMap.toUpdateInputWithCategory(update),
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
			let category = this.aiMap.toCategoryResponse(existing);
			if (category === null) {
				category = await this.ai.categorizeGroupedShoppingListItem(ingredient);
			}

			await this.prisma.shoppingListItem.update({
				data: this.itemMap.toUpdateInputFromRecipeIngerdient(
					ingredient,
					this.aiMap.toCategory(category),
					existing.state === 'BOUGHT'
				),
				where: {
					id: existing.id
				}
			});
			return;
		}

		const category = await this.ai.categorizeGroupedShoppingListItem(ingredient);

		await this.prisma.shoppingListItem.create({
			data: this.itemMap.toCreateInputFromRecipeIngerdient(
				ingredient,
				this.aiMap.toCategory(category)
			)
		});
	}

	async upsertShoppingListItem(update: CreateShoppingListItemWithCategory): Promise<void> {
		await this.prisma.shoppingListItem.upsert({
			create: {
				...this.itemMap.toCreateInput(update)
			},
			update: {
				...this.itemMap.toUpdateInputWithCategory(update)
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
