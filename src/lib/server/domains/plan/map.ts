import type { WeekPlan, WeekPlanMeal } from '$shared/domains/plan/models';
import type { Prisma } from '@prisma/client';
import type { RecipeSummaryMap } from '../recipe/map';
import { Lifecycle, inject, injectable, scoped } from 'tsyringe';

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanMap {
	constructor(@inject('weekPlanMealMap') readonly mealMap: WeekPlanMealMap) {}

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

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanMealMap {
	constructor(@inject('recipeSummaryMap') readonly summaryMap: RecipeSummaryMap) {}

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

	// For now DB & domain are the same
	public toDomain(meal: WeekPlanMeal): WeekPlanMeal {
		return meal;
	}
}
