import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { addRecipeSchema } from '$lib/schemas';
import { redirect, type Actions, fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals: { getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addRecipeSchema);

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

		const form = await superValidate(request, addRecipeSchema);

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
			return fail(500, { form });
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
				return fail(500, { form });
			}
		}

		throw redirect(303, '/browse/recipe');
	}
};
