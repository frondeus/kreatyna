import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fromDB } from '$lib/types';

export const load: PageServerLoad = async ({ locals: { supabase, getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const rawIngredients = async () => {
		const { data: rawIngredients } = await supabase
			.from('raw')
			.select('id, name, container, proteins, carbs, fat, created_by');

		return rawIngredients?.map((r) => fromDB.raw(session, r));
	};

	return {
		session,
		rawIngredients: rawIngredients()
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		const { error } = await supabase.from('raw').delete().eq('id', id);

		if (error) {
			console.error({ error });
			return fail(500, {});
		}

		console.log('Deleted from raw ', id);

		return {};
	}
};

// import { fail, redirect, type Actions } from '@sveltejs/kit';
// import type { PageServerLoad } from './$types';
// import { superValidate } from 'sveltekit-superforms/server';
// import { addRawSchema, addRecipeSchema, deleteRawSchema, editRawSchema } from './schema';
// import { fromDB, type Recipy } from './types';

// export const load: PageServerLoad = async ({ locals: { supabase, getSession } }) => {

// 	const addRawForm = await superValidate(
// 		{
// 			name: '',
// 			container: 100,
// 			proteins: 0,
// 			carbs: 0,
// 			fat: 0
// 		},
// 		addRawSchema
// 	);

// 	const deleteRawForm = await superValidate({ id: 0 }, deleteRawSchema);

// 	const recipies = async (): Promise<Recipy[]> => {
// 		const { data } = await supabase
// 		.from('recipies')
// 		.select('id, name, portions, steps, recipies_ingredients ( id, name, amount, unit, raw ( id, name, container, proteins, carbs, fat, created_by ) )');

// 		const recipies = data ?? [];

// 		return recipies.map(recipy => ({
// 			id: recipy.id,
// 			name: recipy.name,
// 			portions: recipy.portions,
// 			steps: recipy.steps as string[],
// 			ingredients: (recipy.recipies_ingredients ?? []).map(ingredient => ({
// 				id: ingredient.id,
// 				name: ingredient.name,
// 				amount: ingredient.amount,
// 				unit: ingredient.unit ?? undefined,
// 				raw: ingredient.raw ? fromDB.raw(session, ingredient.raw) : undefined
// 			}))
// 		}));
// 	};

// 	return {
// 		session,
// 		addRawForm,
// 		deleteRawForm,
// 		addRecipeForm,
// 		rawIngredients: rawIngredients(),
// 		recipies: recipies()
// 	};
// };

// export const actions: Actions = {
// 	deleteRaw: async ({ request, locals: { supabase, getSession } }) => {
// 		const form = await superValidate(request, deleteRawSchema);

// 		const { error } = await supabase.from('raw').delete().eq('id', form.data.id);

// 		if (error) {
// 			console.log('Failure', { error, request, session });
// 			return fail(500, { form });
// 		}

// 		return { form };
// 	},
// 	editRaw: async ({ request, locals: { supabase, getSession } }) => {
// 		const form = await superValidate(request, editRawSchema);

// 		console.log('POST edit raw', form);

// 		if (!form.valid) {
// 			return fail(400, { form });
// 		}

// 		const session = await getSession();

// 		if (!session) {
// 			return fail(403, { form });
// 		}

// 		const { error } = await supabase.from('raw').upsert({
// 			id: form.data.id,
// 			name: form.data.name,
// 			container: form.data.container,
// 			proteins: form.data.proteins,
// 			carbs: form.data.carbs,
// 			fat: form.data.fat,
// 			created_by: session.user?.id
// 		});

// 		if (error) {
// 			console.log('Failure', { error, request, session });
// 			return fail(500, { form });
// 		}

// 		return { form };
// 	},
// 	addRaw: async ({ request, locals: { supabase, getSession } }) => {
// 		const form = await superValidate(request, addRawSchema);

// 		console.log('POST add raw', form);

// 		if (!form.valid) {
// 			return fail(400, { form });
// 		}

// 		const session = await getSession();

// 		if (!session) {
// 			return fail(403, { form });
// 		}

// 		const { error } = await supabase.from('raw').upsert({
// 			name: form.data.name,
// 			container: form.data.container,
// 			proteins: form.data.proteins,
// 			carbs: form.data.carbs,
// 			fat: form.data.fat,
// 			created_by: session.user?.id
// 		});

// 		if (error) {
// 			console.log('Failure', { error, request, session });
// 			return fail(500, { form });
// 		}

// 		return { form };
// 	},
// 	addRecipe: async({ request, locals: { supabase, getSession }}) => {
// 		const form = await superValidate(request, addRecipeSchema);

// 		console.log('POST add recipe', form);

// 		if(!form.valid) {
// 			return fail(400, { form });
// 		}

// 		const session = await getSession();
// 		if(!session) {
// 			return fail(403, { form });
// 		}

// 		let { error, data } = await supabase.from('recipies')
// 			.insert({
// 				name: form.data.name,
// 				portions: form.data.portions,
// 				steps: form.data.steps
// 			}).select().single();

// 		if (error) {
// 			console.log('Failure recipe', { error, request, session });
// 			return fail(500, { form });
// 		}

// 		const recipeId = data?.id as number;

// 		for(const ingredient of form.data.ingredients) {
// 			({error} = await supabase.from('recipies_ingredients')
// 				.insert({
// 					recipe_id: recipeId,
// 					name: ingredient.name,
// 					amount: ingredient.amount,
// 					unit: ingredient.unit ?? null,
// 					raw_id: ingredient.rawId ?? null
// 				})
// 			);

// 			if (error) {
// 				console.log('Failure recipe ingredient', { error, request, session });
// 				return fail(500, { form });
// 			}
// 		}

// 		return { form }
// 	}
// };
