import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const filter = url.searchParams.getAll('tag');
	const filterTags = filter.map((tag) => Number(tag));

	const recipes = services.recipe.getAll(filterTags);

	const tags = recipes
		.then((recipes) => {
			return recipes.flatMap((recipe) => recipe.tags);
		})
		.then((tags) => {
			const uniqueTags = [...new Map(tags.map((item) => [item.id, item])).values()];

			return uniqueTags;
		});

	return {
		session,
		recipes,
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

		await services.recipe.remove(id);

		return {};
	}
};
