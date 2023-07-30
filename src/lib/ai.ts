export type RecipeResponse = {
    name: string;
    portions: number;
    steps: string[];
    ingredients: Ingredient[]
};

type Ingredient = {
    name: string,
    amount: number,
    unit?: string,
};