import type { PrismaClient } from '@prisma/client';
import type { RecipeMap, RecipeSummaryMap } from './map';
import type {
	Recipe,
	RecipeCreate,
	RecipeSummary,
	RecipeUpdate
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

	async getNotIn(ids: number[], tags: number[]): Promise<RecipeSummary[]> {
		if (tags.length === 0) {
			return await this.getNotInUnfiltered(ids);
		}

		const summaries = await this.prisma.recipe.findMany({
			where: {
				tags: {
					some: {
						id: {
							in: tags
						}
					}
				},
				id: {
					notIn: ids
				}
			},
			select: {
				id: true,
				name: true,
				tags: {
					select: {
						id: true,
						name: true,
						category: {
							select: {
								name: true
							}
						}
					}
				}
			}
		});

		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}

	async getNotInUnfiltered(ids: number[]): Promise<RecipeSummary[]> {
		const summaries = await this.prisma.recipe.findMany({
			select: {
				id: true,
				name: true,
				tags: {
					select: {
						id: true,
						name: true,
						category: {
							select: {
								name: true
							}
						}
					}
				}
			},
			where: {
				id: {
					notIn: ids
				}
			}
		});

		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}

	async getAll(tags: number[]): Promise<RecipeSummary[]> {
		if (tags.length === 0) {
			return await this.getAllUnfiltered();
		}

		const summaries = await this.prisma.recipe.findMany({
			where: {
				tags: {
					some: {
						id: {
							in: tags
						}
					}
				}
			},
			select: {
				id: true,
				name: true,
				tags: {
					select: {
						id: true,
						name: true,
						category: {
							select: {
								name: true
							}
						}
					}
				}
			}
		});

		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}

	private async getAllUnfiltered(): Promise<RecipeSummary[]> {
		const summaries = await this.prisma.recipe.findMany({
			select: {
				id: true,
				name: true,
				tags: {
					select: {
						id: true,
						name: true,
						category: {
							select: {
								name: true
							}
						}
					}
				}
			}
		});

		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}

	async getOne(id: number): Promise<Recipe | null> {
		const recipe = await this.prisma.recipe.findUnique({
			where: {
				id
			},
			include: {
				recipesIngredients: {
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

	async create(recipe: RecipeCreate) {
		await this.prisma.recipe.create({
			data: this.map.toCreateInput(recipe)
		});
	}

	async update(recipe: RecipeUpdate) {
		const ingredientsToKeep = recipe.ingredients.map((i) => i.id).filter(notNullOrUndefined);

		const update = this.map.toUpdateInput(recipe);
		await this.prisma.recipe.update({
			where: { id: recipe.id },
			data: update
		});

		await this.prisma.recipesIngredient.deleteMany({
			where: {
				recipe_id: recipe.id,
				id: {
					notIn: ingredientsToKeep
				}
			}
		});

		for (const ingredient of recipe.ingredients) {
			if (ingredient.id) {
				await this.prisma.recipesIngredient.update({
					data: this.map.ingredientMap.toUpdateInput(recipe.id, ingredient),
					where: {
						id: ingredient.id,
						recipe_id: recipe.id
					}
				});
			} else {
				await this.prisma.recipesIngredient.create({
					data: this.map.ingredientMap.toCreateInput(recipe.id, ingredient)
				});
			}
		}
	}

	async remove(id: number) {
		await this.prisma.recipe.delete({
			where: {
				id
			}
		});
	}
}
