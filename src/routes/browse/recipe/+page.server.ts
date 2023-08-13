import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const recipies = services.recipy.getAll();

	const tags = recipies
		.then((recipies) => {
			return recipies.flatMap((recipy) => recipy.tags);
		})
		.then((tags) => {
			const uniqueTags = [...new Map(tags.map((item) => [item.id, item])).values()];

			return uniqueTags;
		});

	return {
		session,
		recipies,
		related: {
			tags
		}
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { session, services } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		await services.recipy.remove(id);

		return {};
	}
};
