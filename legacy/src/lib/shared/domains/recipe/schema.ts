import { z } from "zod";
import { TagConnectSchema } from "../tags/schema";

export const editIngredientSchema = z.object({
  id: z.number().optional(),
  name: z.string().default(""),
  amount: z.number().positive().default(1),
  unit: z.string().optional(),
  rawId: z.number().optional(),
});
export type EditIngredientSchema = typeof editIngredientSchema;

export const editRecipeSchema = z.object({
  id: z.number(),
  name: z.string().default(""),
  portions: z.number().min(1).default(1),
  ingredients: editIngredientSchema.array(),
  steps: z.string().nonempty().array(),
  tags: TagConnectSchema.array(),
});

export type EditRecipeSchema = typeof editRecipeSchema;

export const addRecipeSchema = editRecipeSchema.extend({
  id: editRecipeSchema.shape.id.optional(),
});

export type AddRecipeSchema = typeof addRecipeSchema;

export const addOrEditRecipeSchema = addRecipeSchema
  .extend({
    name: addRecipeSchema.shape.name.optional(),
    askGPT: z.boolean().default(false).optional(),
  })
  .refine(
    (data) => {
      if (!data.askGPT) {
        return (data.name ?? "").trim().length > 0;
      }
      return true;
    },
    {
      message: "Name is required",
      path: ["name"],
    },
  );

export type AddOrEditRecipeSchema = typeof addOrEditRecipeSchema;
