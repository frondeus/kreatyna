import { CreateShoppingListItemSchema } from '$shared/domains/shopping/schema';
import { error, json, redirect, type RequestHandler } from '@sveltejs/kit';

const BodySchema = CreateShoppingListItemSchema.array();

export const PUT = (async ({ request, locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const body = await request.json();
	const validatedBody = BodySchema.safeParse(body);
	if (validatedBody.success === false) {
		console.error(validatedBody.error.message);
		throw error(400, validatedBody.error.message);
	}

	const service = container.shoppingListService;

	for (const schema of validatedBody.data) {
		await service.upsertShoppingListItem(schema);
	}

	return json('ok');
}) satisfies RequestHandler;
