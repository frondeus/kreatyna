import { redirect, type Actions, error as returnError } from '@sveltejs/kit';
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
			throw returnError(500, 'Could not delete raw');
		}

		console.log('Deleted from raw ', id);

		return {};
	}
};
