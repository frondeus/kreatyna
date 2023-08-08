import { RawMap } from '$lib/server/domains/raw/map';
import { RawService } from '$lib/server/domains/raw/service';
import { RecipeMap, RecipeSummaryMap } from '$lib/server/domains/recipe/map';
import { RecipeService } from '$lib/server/domains/recipe/service';
import { auth } from '$lib/server/lucia';
import { PrismaClient } from '@prisma/client';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const prisma = new PrismaClient();

	const requestAuth = auth.handleRequest(event);
	const session = await requestAuth.validate();

	const services = {
		raw: new RawService(new RawMap(session), prisma),
		recipy: new RecipeService(new RecipeMap(), new RecipeSummaryMap(), prisma)
	};

	event.locals = {
		prisma,
		session,
		services,
		setSession: requestAuth.setSession
	};

	return resolve(event, {
		filterSerializedResponseHeaders(name) {
			return name === 'content-range';
		}
	});
};
