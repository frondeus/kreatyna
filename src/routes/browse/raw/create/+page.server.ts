import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { addRawSchema } from '$lib/schemas';
import { redirect, type Actions, fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals: { session } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const form = superValidate(undefined, addRawSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, services } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, addRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		await services.raw.create(form.data);

		throw redirect(303, '/browse/raw');
	}
};
