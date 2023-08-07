import { json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { RawMap } from '$lib/mappers/raw';

export const GET = (async ({ locals: { auth, prisma } }) => {
	const session = await auth.validate();
	if (!session) {
		throw redirect(303, '/login');
	}

	const map = new RawMap(session);

	const data = await prisma.raw.findMany();
	const raw = data.map((r) => map.toDomain(r));

	return json(raw);
}) satisfies RequestHandler;
