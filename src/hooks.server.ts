import { AIMap } from '$server/domains/ai/map';
import { auth } from '$server/lucia';
import { PrismaClient } from '@prisma/client';
import { RawMap } from '$server/domains/raw/map';
import { RecipeIngredientMap, RecipeMap, RecipeSummaryMap } from '$server/domains/recipe/map';
import { RecipeService } from '$server/domains/recipe/service';
import { TagService } from '$server/domains/tag/service';
import type { Handle } from '@sveltejs/kit';
import { WeekPlanMap, WeekPlanMealMap } from '$server/domains/plan/map';
import { WeekPlanGenerator } from '$server/domains/plan/generator';
import { WeekPlanPopulatorFactory } from '$server/domains/plan/populator';
import { ExistingRecipeSelector } from '$server/domains/plan/selector';
import { RawService } from '$server/domains/raw/service';
import { AIService } from '$server/domains/ai/service';
import { WeekPlanService } from '$server/domains/plan/service';

export const handle: Handle = async ({ event, resolve }) => {
	const prisma = new PrismaClient();

	const requestAuth = auth.handleRequest(event);
	const session = await requestAuth.validate();

	
	const rawMap = new RawMap(session);
	const recipeIngredientMap = new RecipeIngredientMap(rawMap);
	const recipeMap = new RecipeMap(recipeIngredientMap);
	const recipeSummaryMap = new RecipeSummaryMap();
	const weekPlanMealMap = new WeekPlanMealMap(recipeSummaryMap);
	const weekPlanMap = new WeekPlanMap(weekPlanMealMap);
	const aiMap = new AIMap();
	const recipeService = new RecipeService(
		recipeMap,
		recipeSummaryMap,
		prisma
	);
	const tagService = new TagService(prisma);
	const weekPlanGenerator = new WeekPlanGenerator(tagService);
	const existingRecipeSelector = new ExistingRecipeSelector();
	const weekPlanPopulator = new WeekPlanPopulatorFactory(
		tagService,
		recipeSummaryMap,
		recipeService,
		existingRecipeSelector
	);
	const rawService = new RawService(rawMap, prisma);
	const aiService = new AIService(recipeMap, aiMap);
	const weekPlanService = new WeekPlanService(
		prisma,
		weekPlanMap,
		weekPlanGenerator,
		weekPlanPopulator
	);
	const container = {
		prisma,
		session,
		rawMap,
		recipeIngredientMap,
		recipeMap,
		recipeSummaryMap,
		weekPlanMealMap,
		weekPlanMap,
		aiMap,
		recipeService,
		tagService,
		weekPlanGenerator,
		weekPlanPopulator,
		existingRecipeSelector,
		rawService,
		aiService,
		weekPlanService,
	}

	event.locals = {
		prisma,
		session,
		container,
		setSession: requestAuth.setSession
	};

	return resolve(event, {
		filterSerializedResponseHeaders(name) {
			return name === 'content-range';
		}
	});
};
