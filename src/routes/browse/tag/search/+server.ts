import { json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (async ({ locals: { session, services } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const tags = await services.tag.getAll();

	return json(tags);
}) satisfies RequestHandler;
