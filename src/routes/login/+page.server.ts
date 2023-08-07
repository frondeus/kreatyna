import { auth } from '$lib/server/lucia';
import { LuciaError } from 'lucia';
import { fail, redirect } from '@sveltejs/kit';
import { z } from 'zod';
import { superValidate } from 'sveltekit-superforms/server';

import type { Actions, PageServerLoad } from './$types';

const schema = z.object({
	username: z.string(),
	password: z.string()
});

export const load = (async ({ locals }) => {
	const session = await locals.auth.validate();
	if (session) throw redirect(302, '/');
	const form = superValidate(schema);
	return { form };
}) satisfies PageServerLoad;

export const actions: Actions = {
	default: async ({ request, locals }) => {
		const form = await superValidate(request, schema);
		if (!form.valid) {
			return fail(400, { form });
		}

		try {
			const user = await auth.useKey(
				'username',
				form.data.username.toLowerCase(),
				form.data.password
			);
			const session = await auth.createSession({
				userId: user.userId,
				attributes: {}
			});

			locals.auth.setSession(session);
		} catch (e) {
			if (
				e instanceof LuciaError &&
				(e.message === 'AUTH_INVALID_KEY_ID' || e.message === 'AUTH_INVALID_PASSWORD')
			) {
				return fail(400, { message: 'Incorrect username or password' });
			}

			return fail(500, { message: 'An unknown error occurred.' });
		}

		throw redirect(302, '/');
	}
};
