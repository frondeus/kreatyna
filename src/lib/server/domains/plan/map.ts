import type { MealForDay, WeekPlan, WeekPlanMeal } from '$shared/domains/plan/models';
import type { Prisma } from '@prisma/client';
import type { RecipeSummaryMap } from '../recipe/map';

export class WeekPlanMap {
	constructor(readonly mealMap: WeekPlanMealMap) {}

	public toCreateInput(weekPlan: WeekPlan): Prisma.WeekPlanCreateInput {
		return {
			year: weekPlan.year,
			week: weekPlan.week,
			meals: {
				create: weekPlan.meals.map((meal) => this.mealMap.toCreateInputUnconnected(meal))
			}
		};
	}
}

export class WeekPlanMealMap {
	constructor(readonly summaryMap: RecipeSummaryMap) {}

	public toCreateInputUnconnected(
		meal: WeekPlanMeal
	): Prisma.WeekPlanMealCreateWithoutWeekPlanInput {
		const recipe = meal.recipe
			? {
					connect: {
						id: meal.recipe.id
					}
			  }
			: undefined;

		return {
			day: meal.day,
			slot: meal.slot,
			name: meal.name,
			size: meal.size,
			tags: {
				connect: meal.tags.map((t) => ({ id: t.id }))
			},
			recipe
		};
	}

	public toDomain(meal: WeekPlanMeal): MealForDay {
		return {
			name: meal.name,
			day: meal.day,
			slot: { index: meal.slot, size: meal.size },
			tags: meal.recipe?.tags?.map((tag) => tag.id) ?? [],
			recipe: meal.recipe ? this.summaryMap.toDomain(meal.recipe) : undefined
		};
	}
}
