import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { redirect, type Actions, fail, error as returnError } from '@sveltejs/kit';
import * as typechat from 'typechat';
import { PRIVATE_OPENAI_API_KEY } from '$env/static/private';

import aiSchema from '$lib/ai.ts?raw';
import generateRecipePrompt from '$lib/prompts/generateRecipe.txt?raw';
import { addOrEditRecipeSchema } from '$lib/shared/domains/recipe/schema';
import type { RecipeResponse } from '$lib/server/ai';

export const load: PageServerLoad = async ({ locals: { session } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addOrEditRecipeSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, prisma } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		// const debugData = await request.formData();
		// console.log({ steps: debugData.get('steps[0]') });

		const form = await superValidate(request, addOrEditRecipeSchema);

		if (form.data.askGPT) {
			const model = typechat.createOpenAILanguageModel(PRIVATE_OPENAI_API_KEY, 'gpt-3.5-turbo');

			const translator = typechat.createJsonTranslator<RecipeResponse>(
				model,
				aiSchema,
				'RecipeResponse'
			);

			const response = await translator.translate(
				generateRecipePrompt +
					'\n' +
					JSON.stringify({
						ingredients: form.data.ingredients,
						steps: form.data.steps,
						name: form.data.name,
						portions: form.data.portions
					})
			);

			if (!response.success) {
				throw returnError(500, 'GPT Error' + response.message);
			}

			const outcome = response.data;

			form.data.name = outcome.name;
			form.data.steps = outcome.steps;
			form.data.ingredients = outcome.ingredients.map((i) => ({
				name: i.name,
				amount: i.amount,
				unit: i.unit
			}));
			form.data.askGPT = false;

			return { form };
		}

		if (!form.valid) {
			return fail(400, { form });
		}

		const data = await prisma.recipie.create({
			data: {
				name: form.data.name,
				portions: form.data.portions,
				steps: form.data.steps
			},
			select: { id: true }
		});

		const id = data?.id as bigint;

		for (const ingredient of form.data.ingredients) {
			console.log('ingredient', ingredient);
			await prisma.recipiesIngredient.create({
				data: {
					recipe_id: id,
					name: ingredient.name,
					amount: ingredient.amount,
					unit: ingredient.unit ?? null,
					raw_id: ingredient.rawId ?? null
				}
			});
		}

		throw redirect(303, '/browse/recipe');
	}
};
