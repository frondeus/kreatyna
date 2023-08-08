import type {
	Ingredient,
	IngredientUpdate,
	Recipy,
	RecipyCreate,
	RecipyUpdate
} from '$lib/shared/domains/recipe/models';
import type {
	Raw as PrismaRaw,
	Recipie as PrismaRecipie,
	RecipiesIngredient as PrismaRecipiesIngredient,
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
			}))
		};
	}

	public fromSelect(
		dbEntry: PrismaRecipie & {
			recipiesIngredients: (PrismaRecipiesIngredient & { raw: PrismaRaw | null })[];
		}
	): Recipy {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			portions: Number(dbEntry.portions),
			steps: dbEntry.steps as string[],
			ingredients: dbEntry.recipiesIngredients.map((r) => this.ingredientMap.toDomain(r))
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
			steps: domain.steps
		};
	}
}

export class RecipeSummaryMap {
	public toDomain(dbEntry: { id: bigint; name: string }): { id: number; name: string } {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name
		};
	}
}

export class RecipeIngredientMap {
	rawMap: RawMap;

	constructor(rawMap: RawMap) {
		this.rawMap = rawMap;
	}

	public toDomain(dbEntry: PrismaRecipiesIngredient & { raw: PrismaRaw | null }): Ingredient {
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
