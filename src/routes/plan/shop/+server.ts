import { error, json, redirect, type RequestHandler } from '@sveltejs/kit';
import { z } from 'zod';

const BodySchema = z.object({
	week: z.number(),
	year: z.number(),
	meals: z.number().array()
});

export const POST = (async ({ request, locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const body = await request.json();
	const validatedBody = BodySchema.safeParse(body);
	if (validatedBody.success === false) {
		throw error(400, validatedBody.error.message);
	}

	const mealPlanService = container.weekPlanService;
	const service = container.shoppingListService;

	const mealIds = validatedBody.data.meals;
	const ingredients = await mealPlanService.groupRecipeIngredientsLinkedToMeals(mealIds);

	await service.pruneShoppingList();
	await service.createShoppingListItemFromRecipeIngredients(Array.from(ingredients.values()));

	return json('ok');
}) satisfies RequestHandler;
