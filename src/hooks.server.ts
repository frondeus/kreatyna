import { auth } from '$lib/server/lucia';
import { PrismaClient } from '@prisma/client';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const prismaClient = new PrismaClient();
	event.locals.prisma = prismaClient;
	event.locals.auth = auth.handleRequest(event);

	return resolve(event, {
		filterSerializedResponseHeaders(name) {
			return name === 'content-range';
		}
	});
};
