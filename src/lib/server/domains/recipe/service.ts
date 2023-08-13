import type { PrismaClient } from '@prisma/client';
import type { RecipeMap, RecipeSummaryMap } from './map';
import type {
	Recipy,
	RecipyCreate,
	RecipySummary,
	RecipyUpdate
} from '$lib/shared/domains/recipe/models';
import { notNullOrUndefined } from '$lib/shared/utils';

export class RecipeService {
	map: RecipeMap;
	summaryMap: RecipeSummaryMap;
	prisma: PrismaClient;

	constructor(map: RecipeMap, summaryMap: RecipeSummaryMap, prisma: PrismaClient) {
		this.map = map;
		this.prisma = prisma;
		this.summaryMap = summaryMap;
	}
	async getAll(): Promise<RecipySummary[]> {
		const summaries = await this.prisma.recipie.findMany({
			select: { id: true, name: true }
		});
		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}

	async getOne(id: number): Promise<Recipy | null> {
		const recipe = await this.prisma.recipie.findUnique({
			where: {
				id
			},
			include: {
				recipiesIngredients: {
					include: {
						raw: true
					}
				},
				tags: {
					select: {
						name: true,
						categoryId: true,
						id: true,
						category: true
					}
				}
			}
		});

		if (recipe === null) return null;

		return this.map.fromSelect(recipe);
	}

	async create(recipe: RecipyCreate) {
		await this.prisma.recipie.create({
			data: this.map.toCreateInput(recipe)
		});
	}

	async update(recipe: RecipyUpdate) {
		const ingredientsToKeep = recipe.ingredients.map((i) => i.id).filter(notNullOrUndefined);

		const update = this.map.toUpdateInput(recipe);
		await this.prisma.recipie.update({
			where: { id: recipe.id },
			data: update
		});

		await this.prisma.recipiesIngredient.deleteMany({
			where: {
				recipe_id: recipe.id,
				id: {
					notIn: ingredientsToKeep
				}
			}
		});

		for (const ingredient of recipe.ingredients) {
			if (ingredient.id) {
				await this.prisma.recipiesIngredient.update({
					data: this.map.ingredientMap.toUpdateInput(recipe.id, ingredient),
					where: {
						id: ingredient.id,
						recipe_id: recipe.id
					}
				});
			} else {
				await this.prisma.recipiesIngredient.create({
					data: this.map.ingredientMap.toCreateInput(recipe.id, ingredient)
				});
			}
		}
	}

	async remove(id: number) {
		await this.prisma.recipie.delete({
			where: {
				id
			}
		});
	}
}
