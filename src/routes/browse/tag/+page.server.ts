import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	return {
		session,
		tags: services.tag.getAll()
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { session, services } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));

		await services.tag.remove(id);

		return {};
	}
};
