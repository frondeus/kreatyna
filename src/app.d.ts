import type { PrismaClient } from '@prisma/client';
import type * as Icons from 'unplugin-icons/types/svelte';
import type { RawService } from '$server/domains/raw/service';
import type { RecipeService } from '$server/domains/recipe/service';
import type { AIService } from '$server/domains/ai/service';
import type { TagService } from '$server/domains/tag/service';
import type { WeekPlanService } from '$server/domains/plan/service';
import type { RawMap } from '$server/domains/raw/map';
import type { RecipeMap, RecipeSummaryMap } from '$server/domains/recipe/map';
import type { AIMap } from '$server/domains/ai/map';
import type { WeekPlanMap, WeekPlanMealMap } from '$server/domains/plan/map';

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
			services: {
				raw: RawService;
				recipe: RecipeService;
				ai: AIService;
				tag: TagService;
				weekPlan: WeekPlanService;
			};
			maps: {
				raw: RawMap;
				recipe: RecipeMap;
				summary: RecipeSummaryMap;
				ai: AIMap;
				weekPlan: WeekPlanMap;
				weekPlanMeal: WeekPlanMealMap;
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
