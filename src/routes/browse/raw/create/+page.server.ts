import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { addRawSchema } from '$lib/schemas';
import { redirect, type Actions, fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals: { getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addRawSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, addRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		const { error } = await supabase.from('raw').insert({
			name: form.data.name,
			container: form.data.container,
			proteins: form.data.proteins,
			carbs: form.data.carbs,
			fat: form.data.fat,
			created_by: session.user?.id
		});

		if (error) {
			console.error(error);
			return fail(500, { form });
		}

		throw redirect(303, '/browse/raw');
	}
};
