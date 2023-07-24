import { z } from 'zod';

export const editRawSchema = z.object({
	id: z.number().int(),
	name: z.string().default(''),
	container: z.number().default(100),
	proteins: z.number().default(0),
	carbs: z.number().default(0),
	fat: z.number().default(0)
});

export const addRawSchema = editRawSchema.extend({
	id: editRawSchema.shape.id.optional(),
});


export const deleteRawSchema = z.object({
	id: z.number().int()
});

export type DeleteRawSchema = typeof deleteRawSchema;

export const editIngredientSchema = z.object({
		id: z.number(),
		name: z.string().default(''),
		amount: z.number().positive().default(1),
		unit: z.string().optional(),
		rawId: z.number().optional()
});
export type EditIngredientSchema = typeof editIngredientSchema;

export const addIngredientSchema = editIngredientSchema.extend({
		id: editIngredientSchema.shape.id.optional()
});
export type AddIngredientSchema = typeof addIngredientSchema;

export const editRecipeSchema = z.object({
	id: z.number(),
	name: z.string().default(''),
	portions: z.number().min(1).default(1),
	ingredients: editIngredientSchema.array(),
	steps: z.string().nonempty().array()
});

export type EditRecipeSchema = typeof editRecipeSchema;

export const addRecipeSchema = editRecipeSchema.extend({
	id: editRecipeSchema.shape.id.optional(),
	// ingredients: addIngredientSchema.array()
});

export type AddRecipeSchema = typeof addRecipeSchema;

