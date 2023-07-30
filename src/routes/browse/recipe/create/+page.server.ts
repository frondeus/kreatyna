import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { addOrEditRecipeSchema } from '$lib/schemas';
import { redirect, type Actions, fail, error as returnError } from '@sveltejs/kit';
import * as typechat from 'typechat';
import { PRIVATE_OPENAI_API_KEY } from '$env/static/private'

import aiSchema from '$lib/ai.ts?raw';
import generateRecipePrompt from '$lib/prompts/generateRecipe.txt?raw';
import type { RecipeResponse } from '$lib/ai';

export const load: PageServerLoad = async ({ locals: { getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addOrEditRecipeSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		// const debugData = await request.formData();
		// console.log({ steps: debugData.get('steps[0]') });

		const form = await superValidate(request, addOrEditRecipeSchema);

		if (form.data.askGPT) {
			const model = typechat.createOpenAILanguageModel(
				PRIVATE_OPENAI_API_KEY,
				'gpt-3.5-turbo'
			);

			const translator =  typechat.createJsonTranslator<RecipeResponse>(model, aiSchema, 'RecipeResponse');

			const response = await translator.translate(generateRecipePrompt
				+ '\n' + JSON.stringify({
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

		const { error, data } = await supabase
			.from('recipies')
			.insert({
				name: form.data.name,
				portions: form.data.portions,
				steps: form.data.steps
			})
			.select('id')
			.single();

		if (error) {
			console.error(error);
			throw returnError(500, 'Could not create recipe');
		}

		const id = data?.id as number;

		for (const ingredient of form.data.ingredients) {
			console.log('ingredient', ingredient);
			const { error } = await supabase.from('recipies_ingredients').insert({
				recipe_id: id,
				name: ingredient.name,
				amount: ingredient.amount,
				unit: ingredient.unit ?? null,
				raw_id: ingredient.rawId ?? null
			});

			if (error) {
				console.error(error);
				throw returnError(500, 'Could not add ingredient');
			}
		}

		throw redirect(303, '/browse/recipe');
	}
};
