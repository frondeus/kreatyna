import { AIMap } from '$server/domains/ai/map';
import { AIService } from '$server/domains/ai/service';
import { auth } from '$server/lucia';
import { PrismaClient } from '@prisma/client';
import { RawMap } from '$server/domains/raw/map';
import { RawService } from '$server/domains/raw/service';
import { RecipeIngredientMap, RecipeMap, RecipeSummaryMap } from '$server/domains/recipe/map';
import { RecipeService } from '$server/domains/recipe/service';
import { TagService } from '$server/domains/tag/service';
import type { Handle } from '@sveltejs/kit';
import { WeekPlanService } from '$server/domains/plan/service';
import { WeekPlanMap, WeekPlanMealMap } from '$server/domains/plan/map';

export const handle: Handle = async ({ event, resolve }) => {
	const prisma = new PrismaClient();

	const requestAuth = auth.handleRequest(event);
	const session = await requestAuth.validate();

	const rawMap = new RawMap(session);
	const summaryMap = new RecipeSummaryMap();
	const weekPlanMealMap = new WeekPlanMealMap(summaryMap);
	const maps = {
		raw: rawMap,
		recipe: new RecipeMap(new RecipeIngredientMap(rawMap)),
		summary: summaryMap,
		ai: new AIMap(),
		weekPlanMeal: weekPlanMealMap,
		weekPlan: new WeekPlanMap(weekPlanMealMap)
	};
	const raw = new RawService(maps.raw, prisma);
	const recipe = new RecipeService(maps.recipe, maps.summary, prisma);
	const ai = new AIService(maps.recipe, maps.ai);
	const tag = new TagService(prisma);
	const weekPlan = new WeekPlanService(prisma, recipe, tag, maps.summary, maps.weekPlan);

	const services = {
		raw,
		recipe,
		ai,
		tag,
		weekPlan
	};

	event.locals = {
		prisma,
		session,
		services,
		maps,
		setSession: requestAuth.setSession
	};

	return resolve(event, {
		filterSerializedResponseHeaders(name) {
			return name === 'content-range';
		}
	});
};
