import 'reflect-metadata';
import { AIMap } from '$server/domains/ai/map';
import { auth } from '$server/lucia';
import { PrismaClient } from '@prisma/client';
import { RawMap } from '$server/domains/raw/map';
import { RecipeIngredientMap, RecipeMap, RecipeSummaryMap } from '$server/domains/recipe/map';
import { RecipeService } from '$server/domains/recipe/service';
import { TagService } from '$server/domains/tag/service';
import type { Handle } from '@sveltejs/kit';
import { WeekPlanMap, WeekPlanMealMap } from '$server/domains/plan/map';
import { container as globalContainer } from 'tsyringe';
import type { Session } from 'lucia';
import { WeekPlanGenerator } from '$server/domains/plan/generator';
import { WeekPlanPopulatorFactory } from '$server/domains/plan/populator';
import { ExistingRecipeSelector } from '$server/domains/plan/selector';

export const handle: Handle = async ({ event, resolve }) => {
	const prisma = new PrismaClient();

	const requestAuth = auth.handleRequest(event);
	const session = await requestAuth.validate();

	const container = globalContainer.createChildContainer();
	container.register<PrismaClient>('prisma', { useValue: prisma });
	container.register<Session | null>('session', { useValue: session });
	container.register<RawMap>('rawMap', { useClass: RawMap });
	container.register<RecipeIngredientMap>('recipeIngredientMap', { useClass: RecipeIngredientMap });
	container.register<RecipeMap>('recipeMap', { useClass: RecipeMap });
	container.register<RecipeSummaryMap>('recipeSummaryMap', { useClass: RecipeSummaryMap });
	container.register<WeekPlanMealMap>('weekPlanMealMap', { useClass: WeekPlanMealMap });
	container.register<WeekPlanMap>('weekPlanMap', { useClass: WeekPlanMap });
	container.register<AIMap>('aiMap', { useClass: AIMap });
	container.register<RecipeService>('recipeService', { useClass: RecipeService });
	container.register<TagService>('tagService', { useClass: TagService });
	container.register<WeekPlanGenerator>('weekPlanGenerator', { useClass: WeekPlanGenerator });
	container.register<WeekPlanPopulatorFactory>('weekPlanPopulator', {
		useClass: WeekPlanPopulatorFactory
	});
	container.register<ExistingRecipeSelector>('existingRecipeSelector', {
		useClass: ExistingRecipeSelector
	});

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
