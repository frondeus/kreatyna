import { error as returnError, redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { editRawSchema } from '$lib/schemas';

export const load: PageServerLoad = async ({ params, locals: { getSession, supabase } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);

	const { error, data } = await supabase
		.from('raw')
		.select('id, name, container, proteins, carbs, fat, created_by')
		.eq('id', id)
		.maybeSingle();

	if (error) {
		throw returnError(500, error.toString());
	}

	if (data === null) {
		throw returnError(404, 'Raw ingredient was not found');
	}

	const form = superValidate(data, editRawSchema);

	return { session, form, id };
};

export const actions: Actions = {
	default: async ({ request, locals: { supabase, getSession } }) => {
		const session = await getSession();

		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		const { error } = await supabase
			.from('raw')
			.update({
				name: form.data.name,
				container: form.data.container,
				proteins: form.data.proteins,
				carbs: form.data.carbs,
				fat: form.data.fat
			})
			.eq('id', form.data.id);

		if (error) {
			console.error(error);
			throw returnError(500, 'Could not update raw');
		}

		return { form, id: form.data.id };
	}
};
