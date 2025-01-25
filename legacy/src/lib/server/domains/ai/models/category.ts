export type CategoryResponse = {
  categoryId: string;
  categoryName: string;
  categoryEmoji: string;
};

export type CategoryPrompt = {
  name: string;
  ingredients: IngredientPrompt[];
};

export type IngredientPrompt = {
  name: string;
  recipeName: string;
  recipeTags: string[];
  rawName: string | null;
};
