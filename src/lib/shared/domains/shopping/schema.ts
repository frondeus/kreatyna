import { z } from 'zod';

const StateSchema = z.enum(['TODO', 'IN_CART', 'BOUGHT', 'NOT_BUING']);

export const UpdateShoppingListItemSchema = z.object({
	id: z.string().uuid(),
	state: StateSchema.optional(),
	amount: z.number().optional(),
	unit: z.string().optional()
});

export const CreateShoppingListItemSchema = z.object({
	id: z.string().uuid(),
	name: z.string(),
	state: StateSchema,
	amount: z.number(),
	unit: z.string().optional()
});

export type UpdateShoppingListItem = z.infer<typeof UpdateShoppingListItemSchema>;
export type CreateShoppingListItem = z.infer<typeof CreateShoppingListItemSchema>;
