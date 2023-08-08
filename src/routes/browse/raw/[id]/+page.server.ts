import { error as returnError, redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { editRawSchema } from '$lib/schemas';

export const load: PageServerLoad = async ({ params, locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);

	const data = await services.raw.getOne(id);
	if (data === null) {
		throw returnError(404, 'Raw ingredient was not found');
	}

	const form = superValidate(data, editRawSchema);

	return { session, form, id };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, services } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		await services.raw.update(form.data);

		return { form, id: form.data.id };
	}
};
