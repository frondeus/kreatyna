import { redirect, type Actions, error as returnError } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fromDB } from '$lib/types';

export const load: PageServerLoad = async ({ locals: { auth, prisma } }) => {
	const session = await auth.validate();

	if (!session) {
		throw redirect(303, '/login');
	}

	const rawIngredients = async () => {
		const rawIngredients = await prisma.raw.findMany({
			select: {
				id: true,
				name: true,
				container: true,
				proteins: true,
				carbs: true,
				fat: true,
				created_by: true
			}
		});

		return rawIngredients?.map((r) => fromDB.raw(session, r));
	};

	return {
		session,
		rawIngredients: rawIngredients()
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { auth, prisma } }) => {
		const session = await auth.validate();

		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		await prisma.raw.delete({
			where: {
				id
			}
		});

		console.log('Deleted from raw ', id);

		return {};
	}
};
