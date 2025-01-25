import type { z } from "zod";
import type { RawIngredient } from "../raw/models";
import type {
  AddOrEditRecipeSchema,
  AddRecipeSchema,
  EditIngredientSchema,
  EditRecipeSchema,
} from "./schema";
import type { TagConnect, TagSelect } from "../tags/models";
import type { Prisma } from "@prisma/client";
import type {
  ingredients,
  ingredientsWithRecipes,
  recipeSummarySelect,
  recipeWithIngredients,
} from "./queries";

export type RecipeSummarySelect = Prisma.RecipeGetPayload<
  typeof recipeSummarySelect
>;
export type RecipeWithIngredients = Prisma.RecipeGetPayload<
  typeof recipeWithIngredients
>;
export type RecipeIngredient = Prisma.RecipesIngredientGetPayload<
  typeof ingredients
>;
export type RecipeIngredientWithRecipe = Prisma.RecipesIngredientGetPayload<
  typeof ingredientsWithRecipes
>;

export type GroupedRecipeIngredient = Omit<RecipeIngredient, "id"> & {
  ids: bigint[];
};

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

export type IngredientWithRecipe = Ingredient & {
  recipe: RecipeSummary;
};

export interface RecipeFilter {
  tagId: number;
}
