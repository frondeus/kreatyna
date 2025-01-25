// @ts-nocheck
import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms/server';
import { TagCreateOneSchema } from '$shared/domains/tags/schema';

export const load = async ({ locals: { session } }: Parameters<PageServerLoad>[0]) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const form = superValidate(undefined, TagCreateOneSchema);

	return { form, session };
};

export const actions = {
	default: async ({ request, locals: { session, container } }: import('./$types').RequestEvent) => {
		if (!session) {
			throw redirect(303, '/login');
		}
		const form = await superValidate(request, TagCreateOneSchema);

		if (!form.valid) {
			return fail(400, { form });
		}

		const [cat, name] = form.data.name.split(':');

		const tag = container.tagService;

		await tag.create({
			name: name,
			category: cat
		});

		throw redirect(303, '/browse/tag');
	}
};
;null as any as Actions;