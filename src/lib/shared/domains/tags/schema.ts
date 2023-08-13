import { z } from 'zod';

export const TagCreateOneSchema = z.object({
	name: z.string().nonempty()
});

export const TagConnectSchema = z.object({
	id: z.number(),
	name: z.string().optional(),
	category: z.string().optional()
});
