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

export const CategorySchema = z.object({
	id: z.string(),
	name: z.string(),
	emoji: z.string()
});

export type Category = z.infer<typeof CategorySchema>;

export type UpdateShoppingListItem = z.infer<typeof UpdateShoppingListItemSchema>;
export type CreateShoppingListItem = z.infer<typeof CreateShoppingListItemSchema>;

export const CreateShoppingListItemWithCategorySchema = CreateShoppingListItemSchema.extend({
	category: CategorySchema
});

export type CreateShoppingListItemWithCategory = z.infer<
	typeof CreateShoppingListItemWithCategorySchema
>;

export const UpdateShoppingListItemWithCategorySchema = UpdateShoppingListItemSchema.extend({
	category: CategorySchema.optional()
});

export type UpdateShoppingListItemWithCategory = z.infer<
	typeof UpdateShoppingListItemWithCategorySchema
>;
