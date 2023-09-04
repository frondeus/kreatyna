import { error, json, redirect, type RequestHandler } from '@sveltejs/kit';
import { z } from 'zod';

const StateSchema = z.enum(['TODO', 'IN_CART', 'BOUGHT', 'NOT_BUING']);

const BodySchema = z.object({
	id: z.number(),
	state: StateSchema
});

export const PUT = (async ({ request, locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const body = await request.json();
	const validatedBody = BodySchema.safeParse(body);
	if (validatedBody.success === false) {
		throw error(400, validatedBody.error.message);
	}

	const { id, state } = validatedBody.data;

	const service = container.shoppingListService;

	// TODO: Currently prisma is not very good at returning exception when updating a non-existing item.
	// It's easier to check if the item exists before updating it.
	const item = await service.getShoppingListItem(id);
	if (item === null) {
		throw error(404, 'Shopping list item not found');
	}

	await service.updateShoppingListItemState(id, state);

	return json('ok');
}) satisfies RequestHandler;
