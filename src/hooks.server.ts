import { AIMap } from '$lib/server/domains/ai/map';
import { AIService } from '$lib/server/domains/ai/service';
import { RawMap } from '$lib/server/domains/raw/map';
import { RawService } from '$lib/server/domains/raw/service';
import { RecipeIngredientMap, RecipeMap, RecipeSummaryMap } from '$lib/server/domains/recipe/map';
import { RecipeService } from '$lib/server/domains/recipe/service';
import { auth } from '$lib/server/lucia';
import { PrismaClient } from '@prisma/client';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const prisma = new PrismaClient();

	const requestAuth = auth.handleRequest(event);
	const session = await requestAuth.validate();

	const rawMap = new RawMap(session);
	const maps = {
		raw: rawMap,
		recipe: new RecipeMap(new RecipeIngredientMap(rawMap)),
		summary: new RecipeSummaryMap(),
		ai: new AIMap()
	};

	const services = {
		raw: new RawService(maps.raw, prisma),
		recipy: new RecipeService(maps.recipe, maps.summary, prisma),
		ai: new AIService(maps.recipe, maps.ai)
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
