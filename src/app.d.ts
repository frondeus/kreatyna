import type { AIMap } from '$server/domains/ai/map';
import type { AIService } from '$server/domains/ai/service';
import type { WeekPlanGenerator } from '$server/domains/plan/generator';
import type { WeekPlanMap, WeekPlanMealMap } from '$server/domains/plan/map';
import type { WeekPlanPopulatorFactory } from '$server/domains/plan/populator';
import type { ExistingRecipeSelector } from '$server/domains/plan/selector';
import type { WeekPlanService } from '$server/domains/plan/service';
import type { RawMap } from '$server/domains/raw/map';
import type { RawService } from '$server/domains/raw/service';
import type { RecipeIngredientMap, RecipeMap, RecipeSummaryMap } from '$server/domains/recipe/map';
import type { RecipeService } from '$server/domains/recipe/service';
import type { ShoppingListService } from '$server/domains/shopping/service';
import type { TagService } from '$server/domains/tag/service';
import type { PrismaClient } from '@prisma/client';
import type * as Icons from 'unplugin-icons/types/svelte';

declare global {
	interface Date {
		getWeek(): number;
	}
	namespace Lucia {
		type Auth = import('$lib/server/lucia').Auth;
		type DatabaseUserAttributes = {
			username: string;
			is_admin: boolean;
			full_name: string;
		};
		// type DatabaseSessionAttributes = {};
	}
	namespace App {
		interface Locals {
			prisma: PrismaClient;
			session: Session | null;
			setSession: (session: Session | null) => void;
			container: {
				prisma: PrismaClient;
				session: Session | null;
				rawMap: RawMap;
				recipeIngredientMap: RecipeIngredientMap;
				recipeMap: RecipeMap;
				recipeSummaryMap: RecipeSummaryMap;
				weekPlanMealMap: WeekPlanMealMap;
				weekPlanMap: WeekPlanMap;
				aiMap: AIMap;
				recipeService: RecipeService;
				tagService: TagService;
				weekPlanGenerator: WeekPlanGenerator;
				weekPlanPopulator: WeekPlanPopulatorFactory;
				existingRecipeSelector: ExistingRecipeSelector;
				rawService: RawService;
				aiService: AIService;
				weekPlanService: WeekPlanService;
				shoppingListService: ShoppingListService;
			};
		}
		interface PageData {
			session: Session | null;
		}

		// interface Error {}
		// interface Platform {}
	}
}

/// <reference types="@sveltejs/kit" />
/// <reference types="unplugin-icons/types/svelte" />

export default Icons;
export {};
