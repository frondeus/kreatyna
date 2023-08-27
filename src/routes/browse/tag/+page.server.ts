import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { TagService } from '$server/domains/tag/service';

export const load: PageServerLoad = async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const tag = container.resolve(TagService);

	return {
		session,
		tags: tag.getAll()
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { session, container } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		const tag = container.resolve(TagService);
		await tag.remove(id);

		return {};
	}
};
