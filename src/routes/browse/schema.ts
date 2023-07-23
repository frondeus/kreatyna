import { z } from 'zod';

export const addRawSchema = z.object({
	name: z.string().default(''),
	container: z.number().default(100),
	proteins: z.number().default(0),
	carbs: z.number().default(0),
	fat: z.number().default(0)
});

export const editRawSchema = z.object({
	id: z.number().int(),
	name: z.string(),
	container: z.number(),
	proteins: z.number(),
	carbs: z.number(),
	fat: z.number()
});

export const deleteRawSchema = z.object({
	id: z.number().int()
});

export type DeleteRawSchema = typeof deleteRawSchema;

export const addIngredientSchema = z.object({
	
		name: z.string().default(''),
		amount: z.number().positive().default(1),
		unit: z.string().optional(),
		rawId: z.number().optional()
});
export type AddIngredientSchema = typeof addIngredientSchema;

export const addRecipeSchema = z.object({
	name: z.string().default(''),
	portions: z.number().min(1).default(1),
	ingredients: addIngredientSchema.array(),
	steps: z.string().nonempty().array()
});

export type AddRecipeSchema = typeof addRecipeSchema;
