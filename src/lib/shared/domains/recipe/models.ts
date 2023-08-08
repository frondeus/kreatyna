import type { z } from 'zod';
import type { RawIngredient } from '../raw/models';
import type {
	AddOrEditRecipeSchema,
	AddRecipeSchema,
	EditIngredientSchema,
	EditRecipeSchema
} from './schema';

export interface WithIngredients {
	ingredients: WithRawAndAmount[];
}

export interface RecipySummary {
	id: number;
	name: string;
}
export interface Recipy {
	id: number;
	name: string;
	portions: number;
	steps: string[];
	ingredients: Ingredient[];
}

export type RecipeCreateOrUpdate = z.infer<AddOrEditRecipeSchema>;
export type RecipyCreate = z.infer<AddRecipeSchema>;
export type RecipyUpdate = z.infer<EditRecipeSchema>;
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
