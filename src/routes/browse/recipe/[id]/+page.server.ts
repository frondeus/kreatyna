import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { fail, redirect, error as returnError, type Actions } from '@sveltejs/kit';
import { editRecipeSchema } from '$lib/schemas';
import { fromDB } from '$lib/types';

function notNull<T>(value: T | null): value is T {
	return value !== null;
}

export const load: PageServerLoad = async ({ params, locals: { getSession, supabase } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);

	const { error, data } = await supabase
		.from('recipies')
		.select(
			'id, name, portions, steps, recipies_ingredients ( id, name, amount, unit, raw ( id, name, container, proteins, carbs, fat, created_by  ) )'
		)
		.eq('id', id)
		.order('id', { foreignTable: 'recipies_ingredients', ascending: true })
		.maybeSingle();

	if (error) {
		throw returnError(500, error.toString());
	}

	if (data === null) {
		throw returnError(404, 'Recipe was not found');
	}

	const ingredients = data.recipies_ingredients ?? [];
	const transformedData = {
		id: data.id,
		name: data.name,
		portions: data.portions,
		steps: data.steps as string[],
		ingredients: ingredients.map((raw) => ({
			id: raw.id,
			name: raw.name,
			amount: raw.amount,
			unit: raw.unit ?? undefined,
			// raw: raw.raw ? fromDB.raw(session, raw.raw) : undefined
			rawId: raw.raw?.id ?? undefined
		}))
	};

	const relatedIngredients = new Map(
		ingredients
			.map((i) => i.raw)
			.filter(notNull)
			.map((raw) => fromDB.raw(session, raw))
			.map((raw) => [raw.id, raw] as const)
	);

	const form = superValidate(transformedData, editRecipeSchema);

	return { session, form, id, relatedIngredients };
};

export const actions: Actions = {
	default: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRecipeSchema);

		if (!form.valid) {
			console.error(form);
			return fail(400, { form });
		}

		let { error } = await supabase
			.from('recipies')
			.update({
				name: form.data.name,
				portions: form.data.portions,
				steps: form.data.steps
			})
			.eq('id', form.data.id);

		if (error) {
			console.error(error);
			throw returnError(500, 'Could not save recipy');
		}

		const ingredientsToKeep = form.data.ingredients
			.filter((i) => i.id !== undefined)
			.map((i) => i.id)
			.join(',');

		({ error } = await supabase
			.from('recipies_ingredients')
			.delete()
			.eq('recipe_id', form.data.id)
			.not('id', 'in', '(' + ingredientsToKeep + ')'));

		if (error) {
			console.error(error);
			throw returnError(500, 'Could not remove ingredient');
		}

		for (const ingredient of form.data.ingredients) {
			console.log('ingredient', ingredient);
			if (ingredient.id) {
				const { error } = await supabase
					.from('recipies_ingredients')
					.update({
						name: ingredient.name,
						amount: ingredient.amount,
						unit: ingredient.unit ?? null,
						raw_id: ingredient.rawId ?? null
					})
					.eq('id', ingredient.id)
					.eq('recipe_id', form.data.id);

				if (error) {
					console.error(error);
					throw returnError(500, 'Could not update ingredient');
				}
			} else {
				const { error } = await supabase.from('recipies_ingredients').insert({
					recipe_id: form.data.id,
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
		}

		return { form };
	}
};
