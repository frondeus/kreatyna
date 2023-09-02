import type {
	Ingredient,
	IngredientUpdate,
	RecipeCreateOrUpdate,
	RecipeSummarySelect,
	Recipe,
	RecipeCreate,
	RecipeSummary,
	RecipeUpdate
} from '$lib/shared/domains/recipe/models';
import type {
	Raw as PrismaRaw,
	Recipe as PrismaRecipe,
	RecipesIngredient as PrismaRecipesIngredient,
	Tag as PrismaTag,
	TagCategory as PrismaTagCategory,
	Prisma
} from '@prisma/client';
import type { RawMap } from '../raw/map';
import type { RecipeResponse } from '../ai/models';

export class RecipeMap {
	constructor(readonly ingredientMap: RecipeIngredientMap) {}

	public fromAI(model: RecipeResponse): RecipeCreate {
		return {
			name: model.name,
			steps: model.steps,
			portions: model.portions,
			ingredients: model.ingredients.map((i) => ({
				name: i.name,
				amount: i.amount,
				unit: i.unit
			})),
			tags: []
		};
	}

	public fromSelect(
		dbEntry: PrismaRecipe & {
			recipesIngredients: (PrismaRecipesIngredient & { raw: PrismaRaw | null })[];
			tags: (PrismaTag & { category: PrismaTagCategory })[];
		}
	): Recipe {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			portions: Number(dbEntry.portions),
			steps: dbEntry.steps as string[],
			tags: dbEntry.tags.map((t) => ({
				id: t.id,
				name: t.name,
				category: t.category.name
			})),
			ingredients: dbEntry.recipesIngredients.map((r) => this.ingredientMap.toDomain(r))
		};
	}

	public toFormSchema(recipe: Recipe): RecipeCreateOrUpdate {
		return {
			id: recipe.id,
			name: recipe.name,
			portions: recipe.portions,
			steps: recipe.steps,
			tags: recipe.tags,
			ingredients: recipe.ingredients.map((i) => ({
				name: i.name,
				amount: i.amount,
				unit: i.unit,
				rawId: i.raw?.id
			})),
			askGPT: false
		};
	}

	public toCreateInput(
		domain: RecipeCreate & { ingredients: IngredientUpdate[] }
	): Prisma.RecipeCreateInput {
		return {
			name: domain.name,
			portions: domain.portions,
			steps: domain.steps,
			recipesIngredients: {
				create: domain.ingredients.map((i) => this.ingredientMap.toCreateInputUnconnected(i))
			}
		};
	}

	public toUpdateInput(domain: RecipeUpdate): Prisma.RecipeUpdateInput {
		return {
			name: domain.name,
			portions: domain.portions,
			steps: domain.steps,
			tags: {
				set:
					domain.tags.length > 0
						? domain.tags.map((t) => {
								const { id } = t;
								return { id };
						  })
						: undefined
			}
		};
	}
}

export class RecipeSummaryMap {
	public toDomain(dbEntry: RecipeSummarySelect): RecipeSummary {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			tags: dbEntry.tags.map((t) => ({
				id: t.id,
				name: t.name,
				category: t.category.name
			}))
		};
	}
	public toDb(domain: RecipeSummary): RecipeSummarySelect {
		return {
			id: BigInt(domain.id),
			name: domain.name,
			tags: domain.tags.map((t) => ({
				id: t.id,
				name: t.name,
				category: {
					name: t.category
				}
			}))
		};
	}
}

export class RecipeIngredientMap {
	constructor( readonly rawMap: RawMap) {}

	public toDomain(
		dbEntry: PrismaRecipesIngredient & {
			raw: PrismaRaw | null;
		}
	): Ingredient {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			amount: dbEntry.amount.toNumber(),
			unit: dbEntry.unit ?? undefined,
			raw: dbEntry.raw ? this.rawMap.toDomain(dbEntry.raw) : undefined
		};
	}

	public toCreateInputUnconnected(
		domain: IngredientUpdate
	): Prisma.RecipesIngredientCreateWithoutRecipesInput {
		return {
			name: domain.name,
			amount: domain.amount,
			unit: domain.unit ?? null,
			raw: domain.rawId
				? {
						connect: {
							id: domain.rawId
						}
				  }
				: undefined
		};
	}
	public toCreateInput(
		recipe_id: number,
		domain: IngredientUpdate
	): Prisma.RecipesIngredientCreateInput {
		return {
			...this.toCreateInputUnconnected(domain),
			recipes: {
				connect: {
					id: recipe_id
				}
			}
		};
	}

	public toUpdateInput(
		recipe_id: number,
		domain: IngredientUpdate
	): Prisma.RecipesIngredientUpdateInput {
		return {
			name: domain.name,
			amount: domain.amount,
			unit: domain.unit ?? null,
			recipes: {
				connect: {
					id: recipe_id
				}
			},
			raw: domain.rawId
				? {
						connect: {
							id: domain.rawId
						}
				  }
				: undefined
		};
	}
}
