import { json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { RawService } from '$server/domains/raw/service';

export const GET = (async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const service = container.resolve(RawService);
	const raw = await service.getAll();

	return json(raw);
}) satisfies RequestHandler;
