import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { redirect, type Actions, fail, error as returnError } from '@sveltejs/kit';

import { addOrEditRecipeSchema } from '$lib/shared/domains/recipe/schema';

export const load: PageServerLoad = async ({ locals: { session } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addOrEditRecipeSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, services } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, addOrEditRecipeSchema);

		if (form.data.askGPT) {
			try {
				const outcome = await services.ai.getRecipe(form.data);
				form.data = {
					...form.data,
					...outcome,
					askGPT: false
				};
			} catch (e) {
				let message = 'Unknown Error';
				if (e instanceof Error) {
					message = e.message;
				}
				throw returnError(500, message);
			}

			return { form };
		}

		if (!form.valid) {
			return fail(400, { form });
		}

		const data = {
			...form.data,
			// addOrEditRecipeSchema ensures that name must be defined and not empty
			name: form.data.name as string
		};

		await services.recipy.create(data);

		throw redirect(303, '/browse/recipe');
	}
};
