export type RecipeResponse = {
  name: string;
  portions: number;
  steps: string[];
  ingredients: Ingredient[];
};

export type Ingredient = {
  name: string;
  amount: number;
  unit?: "g" | "ml" | "l" | "kg" | "tsp" | "tbsp" | "pn";
};
