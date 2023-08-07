import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { addRawSchema } from '$lib/schemas';
import { redirect, type Actions, fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals: { auth } }) => {
	const session = await auth.validate();

	if (!session) {
		throw redirect(303, '/login');
	}

	const form = await superValidate(undefined, addRawSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { auth, prisma } }) => {
		const session = await auth.validate();

		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, addRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		await prisma.raw.create({
			data: {
				name: form.data.name,
				container: form.data.container,
				proteins: form.data.proteins,
				carbs: form.data.carbs,
				fat: form.data.fat,
				created_by: session.user?.userId
			}
		});

		throw redirect(303, '/browse/raw');
	}
};
