import { z } from 'zod';

export const addRawSchema = z.object({
	name: z.string(),
	container: z.number().default(100),
	proteins: z.number(),
	carbs: z.number(),
	fat: z.number()
});

export const editRawSchema = z.object({
	id: z.number().int(),
	name: z.string(),
	container: z.number().default(100),
	proteins: z.number(),
	carbs: z.number(),
	fat: z.number()
});

export const deleteRawSchema = z.object({
	id: z.number().int()
});

export type DeleteRawSchema = typeof deleteRawSchema;
