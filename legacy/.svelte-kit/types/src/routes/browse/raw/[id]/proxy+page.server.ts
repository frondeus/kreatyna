// @ts-nocheck
import { error as returnError, redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { editRawSchema } from '$lib/shared/domains/raw/schema';

export const load = async ({ params, locals: { session, container } }: Parameters<PageServerLoad>[0]) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);
	const raw = container.rawService;

	const data = await raw.getOne(id);
	if (data === null) {
		throw returnError(404, 'Raw ingredient was not found');
	}

	const form = superValidate(data, editRawSchema);

	return { session, form, id };
};

export const actions = {
	default: async ({ request, locals: { session, container } }: import('./$types').RequestEvent) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		const raw = container.rawService;

		await raw.update(form.data);

		return { form, id: form.data.id };
	}
};
;null as any as Actions;