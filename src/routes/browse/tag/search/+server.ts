import { json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { TagService } from '$server/domains/tag/service';

export const GET = (async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const tag = container.resolve(TagService);
	const tags = await tag.getAll();

	return json(tags);
}) satisfies RequestHandler;
