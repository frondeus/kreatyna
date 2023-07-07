import { fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms/server';

import { z } from 'zod';

const updateSchema = z.object({
	fullName: z.string().default('John Doe').optional(),
	username: z.string()
});

const signoutSchema = z.object({});

export const load = async ({ locals: { supabase, getSession } }) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/');
	}

	const { data: profile } = await supabase
		.from('profiles')
		.select(`username, full_name`)
		.eq('id', session.user.id)
		.single();

	const updateForm = await superValidate(
		{
			username: profile.username,
			fullName: profile.full_name
		},
		updateSchema
	);
	console.log({ profile, updateForm });
	const signoutForm = await superValidate(signoutSchema);

	return { session, updateForm, signoutForm };
};

export const actions = {
	update: async ({ request, locals: { supabase, getSession } }) => {
		const form = await superValidate(request, updateSchema);

		console.log('POST update', form);

		if (!form.valid) {
			return fail(400, { form });
		}

		const session = await getSession();

		const { error } = await supabase.from('profiles').upsert({
			id: session?.user.id,
			full_name: form.data.fullName,
			username: form.data.username,
			updated_at: new Date()
		});

		if (error) {
			console.log('Failure', { error, request, session });
			return fail(500, {
				form
			});
		}

		return {
			form
		};
	},
	signout: async ({ locals: { supabase, getSession } }) => {
		console.log('Signout action');
		const session = await getSession();
		if (session) {
			await supabase.auth.signOut();
			throw redirect(303, '/');
		}
	}
};
