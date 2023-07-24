import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals: { getSession, supabase } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const recipies = async () => {
		const { data } = await supabase.from('recipies').select('id, name');

		return data ?? [];
	};

	return { session, recipies: recipies() };
};

export const actions: Actions = {
	delete: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		const { error } = await supabase.from('recipies').delete().eq('id', id);

		if (error) {
			console.error({ error });
			return fail(500, {});
		}

		console.log('Deleted from recipies ', id);

		return {};
	}
};
