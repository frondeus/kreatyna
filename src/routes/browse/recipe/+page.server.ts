import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { RecipeSummaryMap } from '$lib/mappers/recipe';

export const load: PageServerLoad = async ({ locals: { auth, prisma } }) => {
	const session = await auth.validate();

	if (!session) {
		throw redirect(303, '/login');
	}

	const map = new RecipeSummaryMap();

	const recipies = async () => {
		const data = await prisma.recipie.findMany({
			select: { id: true, name: true }
		});

		return data.map((r) => map.toDomain(r)) ?? [];
	};

	return { session, recipies: recipies() };
};

export const actions: Actions = {
	delete: async ({ request, locals: { auth, prisma } }) => {
		const session = await auth.validate();

		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		await prisma.recipie.delete({ where: { id } });

		console.log('Deleted from recipies ', id);

		return {};
	}
};
