import type { RawIngredient } from '../raw/models';

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
