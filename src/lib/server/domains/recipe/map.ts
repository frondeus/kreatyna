import type {
	Ingredient,
	IngredientUpdate,
	RecipeCreateOrUpdate,
	RecipeSummarySelect,
	Recipy,
	RecipyCreate,
	RecipySummary,
	RecipyUpdate
} from '$lib/shared/domains/recipe/models';
import type {
	Raw as PrismaRaw,
	Recipie as PrismaRecipie,
	RecipiesIngredient as PrismaRecipiesIngredient,
	Tag as PrismaTag,
	TagCategory as PrismaTagCategory,
	Prisma
} from '@prisma/client';
import type { RawMap } from '../raw/map';
import type { RecipeResponse } from '../ai/models';

export class RecipeMap {
	ingredientMap: RecipeIngredientMap;
	constructor(ingredientMap: RecipeIngredientMap) {
		this.ingredientMap = ingredientMap;
	}

	public fromAI(model: RecipeResponse): RecipyCreate {
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
		dbEntry: PrismaRecipie & {
			recipiesIngredients: (PrismaRecipiesIngredient & { raw: PrismaRaw | null })[];
			tags: (PrismaTag & { category: PrismaTagCategory })[];
		}
	): Recipy {
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
			ingredients: dbEntry.recipiesIngredients.map((r) => this.ingredientMap.toDomain(r))
		};
	}

	public toFormSchema(recipy: Recipy): RecipeCreateOrUpdate {
		return {
			id: recipy.id,
			name: recipy.name,
			portions: recipy.portions,
			steps: recipy.steps,
			tags: recipy.tags,
			ingredients: recipy.ingredients.map((i) => ({
				name: i.name,
				amount: i.amount,
				unit: i.unit,
				rawId: i.raw?.id
			})),
			askGPT: false
		};
	}

	public toCreateInput(
		domain: RecipyCreate & { ingredients: IngredientUpdate[] }
	): Prisma.RecipieCreateInput {
		return {
			name: domain.name,
			portions: domain.portions,
			steps: domain.steps,
			recipiesIngredients: {
				create: domain.ingredients.map((i) => this.ingredientMap.toCreateInputUnconnected(i))
			}
		};
	}

	public toUpdateInput(domain: RecipyUpdate): Prisma.RecipieUpdateInput {
		return {
			name: domain.name,
			portions: domain.portions,
			steps: domain.steps,
			tags: {
				deleteMany: {},
				connect:
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
	public toDomain(dbEntry: RecipeSummarySelect): RecipySummary {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			tags: dbEntry.tags.map((t) => ({
				id: Number(t.id),
				name: t.name,
				category: t.category.name
			}))
		};
	}
}

export class RecipeIngredientMap {
	rawMap: RawMap;

	constructor(rawMap: RawMap) {
		this.rawMap = rawMap;
	}

	public toDomain(
		dbEntry: PrismaRecipiesIngredient & {
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
	): Prisma.RecipiesIngredientCreateWithoutRecipiesInput {
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
	): Prisma.RecipiesIngredientCreateInput {
		return {
			...this.toCreateInputUnconnected(domain),
			recipies: {
				connect: {
					id: recipe_id
				}
			}
		};
	}

	public toUpdateInput(
		recipe_id: number,
		domain: IngredientUpdate
	): Prisma.RecipiesIngredientUpdateInput {
		return {
			name: domain.name,
			amount: domain.amount,
			unit: domain.unit ?? null,
			recipies: {
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
