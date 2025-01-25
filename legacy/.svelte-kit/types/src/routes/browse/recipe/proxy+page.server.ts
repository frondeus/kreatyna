// @ts-nocheck
import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = async ({ url, locals: { session, container } }: Parameters<PageServerLoad>[0]) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const filter = url.searchParams.getAll('tag');
	const filterTags = filter.map((tag) => Number(tag));

	const recipe = container.recipeService;
	const recipes = recipe.getAll(filterTags);

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

export const actions = {
	delete: async ({ request, locals: { session, container } }: import('./$types').RequestEvent) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		const recipe = container.recipeService;
		await recipe.remove(id);

		return {};
	}
};
;null as any as Actions;