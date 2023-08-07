import { error as returnError, redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { editRawSchema } from '$lib/schemas';
import { RawMap } from '$lib/mappers/raw';

export const load: PageServerLoad = async ({ params, locals: { auth, prisma } }) => {
	const session = await auth.validate();
	const map = new RawMap(session);

	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);

	const data = await prisma.raw.findUnique({
		where: { id }
	});

	if (data === null) {
		throw returnError(404, 'Raw ingredient was not found');
	}

	const form = superValidate(map.toDomain(data), editRawSchema);

	return { session, form, id };
};

export const actions: Actions = {
	default: async ({ request, locals: { auth, prisma } }) => {
		const session = await auth.validate();

		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRawSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		await prisma.raw.update({
			where: { id: form.data.id },
			data: {
				name: form.data.name,
				container: form.data.container,
				proteins: form.data.proteins,
				carbs: form.data.carbs,
				fat: form.data.fat
			}
		});

		return { form, id: form.data.id };
	}
};
