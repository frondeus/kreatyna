import { json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (async ({ locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const raw = await services.raw.getAll();

	return json(raw);
}) satisfies RequestHandler;
