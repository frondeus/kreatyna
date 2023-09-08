import {
	CreateShoppingListItemSchema,
	type CreateShoppingListItem
} from '$shared/domains/shopping/schema';
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

	const processItem = async (schema: CreateShoppingListItem) => {
		const item = await service.getShoppingListItem(schema.id);
		let category = container.aiMap.toCategoryResponse(item);

		if (category === null) {
			category = await (item === null
				? container.aiService.categorizeUnlinkedShoppingListItem(schema.name)
				: container.aiService.categorizeLinkedShoppingListItem(item));
		}

		const schemaWithCategory = {
			...schema,
			category: container.aiMap.toCategory(category)
		};

		await service.upsertShoppingListItem(schemaWithCategory);
	};

	await Promise.all(validatedBody.data.map((i) => processItem(i)));

	return json('ok');
}) satisfies RequestHandler;
