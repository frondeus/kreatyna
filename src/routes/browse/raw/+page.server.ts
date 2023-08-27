import { redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { RawService } from '$server/domains/raw/service';

export const load: PageServerLoad = async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const raw = container.resolve(RawService);

	return {
		session,
		rawIngredients: raw.getAll()
	};
};

export const actions: Actions = {
	delete: async ({ request, locals: { session, container } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const data = await request.formData();
		const id = Number(data.get('id'));
		const raw = container.resolve(RawService);

		await raw.remove(id);

		return {};
	}
};
