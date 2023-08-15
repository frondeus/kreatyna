import type { z } from 'zod';
import type { RawIngredient } from '../raw/models';
import type {
	AddOrEditRecipeSchema,
	AddRecipeSchema,
	EditIngredientSchema,
	EditRecipeSchema
} from './schema';
import type { TagConnect, TagSelect } from '../tags/models';
import { Prisma } from '@prisma/client';

const recipeSummarySelect = Prisma.validator<Prisma.RecipeFindManyArgs>()({
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
export type RecipeSummarySelect = Prisma.RecipeGetPayload<typeof recipeSummarySelect>;

export interface WithIngredients {
	ingredients: WithRawAndAmount[];
}

export interface RecipeSummary {
	id: number;
	name: string;
	tags: TagSelect[];
}
export interface Recipe {
	id: number;
	name: string;
	portions: number;
	steps: string[];
	ingredients: Ingredient[];
	tags: TagConnect[];
}

export type RecipeCreateOrUpdate = z.infer<AddOrEditRecipeSchema>;
export type RecipeCreate = z.infer<AddRecipeSchema>;
export type RecipeUpdate = z.infer<EditRecipeSchema>;
export type IngredientUpdate = z.infer<EditIngredientSchema>;

export interface WithRawAndAmount {
	raw?: RawIngredient;
	amount: number;
}

export interface Ingredient {
	id: number;
	name: string;
	unit?: string;
	amount: number;
	raw?: RawIngredient;
}

export interface RecipeFilter {
	tagId: number;
}
