export interface Nutrients {
	proteins: number;
	carbs: number;
	fat: number;
}

export interface RawIngredient extends Nutrients {
	id: number;
	name: string;
	container: number;
	createdByMe: boolean | undefined;
}

export interface WithIngredients {
	ingredients: WithRawAndAmount[];
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
