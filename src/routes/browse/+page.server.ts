import { fail, redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { addRawSchema, deleteRawSchema, editRawSchema } from './schema';
import { fromDB } from './types';

export const load: PageServerLoad = async ({ locals: { supabase, getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const { data: rawIngredients } = await supabase
		.from('raw')
		.select('id, name, container, proteins, carbs, fat, created_by');

	const addRawForm = await superValidate(
		{
			name: '',
			container: 100,
			proteins: 0,
			carbs: 0,
			fat: 0
		},
		addRawSchema
	);

	const deleteRawForm = await superValidate({ id: 0 }, deleteRawSchema);

	return {
		session,
		addRawForm,
		deleteRawForm,
		rawIngredients: rawIngredients?.map((r) => fromDB(session, r))
	};
};

export const actions: Actions = {
	deleteRaw: async ({ request, locals: { supabase, getSession } }) => {
		const form = await superValidate(request, deleteRawSchema);

		const session = await getSession();

		if (!session) {
			return fail(403, {});
		}
		const { error } = await supabase.from('raw').delete().eq('id', form.data.id);

		if (error) {
			console.log('Failure', { error, request, session });
			return fail(500, { form });
		}

		return { form };
	},
	editRaw: async ({ request, locals: { supabase, getSession } }) => {
		const form = await superValidate(request, editRawSchema);

		console.log('POST edit raw', form);

		if (!form.valid) {
			return fail(400, { form });
		}

		const session = await getSession();

		if (!session) {
			return fail(403, { form });
		}

		const { error } = await supabase.from('raw').upsert({
			id: form.data.id,
			name: form.data.name,
			container: form.data.container,
			proteins: form.data.proteins,
			carbs: form.data.carbs,
			fat: form.data.fat,
			created_by: session.user?.id
		});

		if (error) {
			console.log('Failure', { error, request, session });
			return fail(500, { form });
		}

		return { form };
	},
	addRaw: async ({ request, locals: { supabase, getSession } }) => {
		const form = await superValidate(request, addRawSchema);

		console.log('POST add raw', form);

		if (!form.valid) {
			return fail(400, { form });
		}

		const session = await getSession();

		if (!session) {
			return fail(403, { form });
		}

		const { error } = await supabase.from('raw').upsert({
			name: form.data.name,
			container: form.data.container,
			proteins: form.data.proteins,
			carbs: form.data.carbs,
			fat: form.data.fat,
			created_by: session.user?.id
		});

		if (error) {
			console.log('Failure', { error, request, session });
			return fail(500, { form });
		}

		return { form };
	}
};
