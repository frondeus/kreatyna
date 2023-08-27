import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { TagCreateOneSchema } from '$shared/domains/tags/schema';
import { TagService } from '$server/domains/tag/service';

export const load: PageServerLoad = async ({ locals: { session } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const form = superValidate(undefined, TagCreateOneSchema);

	return { form, session };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, container } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}
		const form = await superValidate(request, TagCreateOneSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		const [cat, name] = form.data.name.split(':');

		const tag = container.resolve(TagService);

		await tag.create({
			name: name,
			category: cat
		});

		throw redirect(303, '/browse/tag');
	}
};
