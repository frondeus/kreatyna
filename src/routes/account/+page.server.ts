import {
	fail,
	redirect,
	error as returnError,
	type Actions,
	type RequestEvent
} from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms/server';

import { z } from 'zod';

const updateSchema = z.object({
	fullName: z.string().default('John Doe').optional()
});

const signoutSchema = z.object({});

export const load = async ({ locals: { supabase, getSession } }: RequestEvent) => {
	const session = await getSession();

	if (!session) {
		throw redirect(303, '/');
	}

	const { data: profile } = await supabase
		.from('profiles')
		.select(`full_name`)
		.eq('id', session.user.id)
		.single();

	const updateForm = await superValidate(
		{
			fullName: profile?.full_name ?? undefined
		},
		updateSchema
	);
	console.log({ profile, updateForm });
	const signoutForm = await superValidate(signoutSchema);

	return { session, updateForm, signoutForm };
};

export const actions: Actions = {
	update: async ({ request, locals: { supabase, getSession } }) => {
		const form = await superValidate(request, updateSchema);

		console.log('POST update', form);

		if (!form.valid) {
			return fail(400, { form });
		}

		const session = await getSession();

		if (!session) {
			return redirect(303, '/login');
		}

		const { error } = await supabase.from('profiles').upsert({
			id: session.user.id,
			full_name: form.data.fullName,
			updated_at: new Date().toISOString()
		});

		if (error) {
			console.log('Failure', { error, request, session });
			throw returnError(500, 'Failed to update profile');
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
