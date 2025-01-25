import type {
	WeekPlanWithId,
	WeekPlanMealWithId,
	WeekPlanCreate,
	WeekPlanMealWithoutId
} from '$shared/domains/plan/models';
import type { Prisma } from '@prisma/client';
import type { RecipeSummaryMap } from '../recipe/map';

export class WeekPlanMap {
	constructor(readonly mealMap: WeekPlanMealMap) {}

	public toUpdateInput(weekPlan: WeekPlanWithId): Prisma.WeekPlanUpdateInput {
		return {
			year: weekPlan.year,
			week: weekPlan.week,
			meals: {
				deleteMany: {
					id: {
						notIn: weekPlan.meals.map((meal) => meal.id)
					}
				},
				upsert: weekPlan.meals.map((meal) => this.mealMap.toUpsertInputUnconnected(meal))
			}
		};
	}

	public toCreateInput(weekPlan: WeekPlanCreate): Prisma.WeekPlanCreateInput {
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

	public toUpsertInputUnconnected(
		meal: WeekPlanMealWithId
	): Prisma.WeekPlanMealUpsertWithWhereUniqueWithoutWeekPlanInput {
		const recipe = meal.recipe
			? {
					connect: {
						id: meal.recipe.id
					}
			  }
			: undefined;

		return {
			where: {
				id: meal.id
			},
			update: {
				day: meal.day,
				slot: meal.slot,
				name: meal.name,
				size: meal.size,
				tags: {
					set: meal.tags.map((t) => ({ id: t.id }))
				},
				recipe
			},
			create: this.toCreateInputUnconnected(meal)
		};
	}

	public toCreateInputUnconnected(
		meal: WeekPlanMealWithoutId
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
	public toDomain(meal: WeekPlanMealWithId): WeekPlanMealWithId {
		return meal;
	}
}
