import { Prisma } from '@prisma/client';
import { tagWithCategory } from '../tags/queries';
import { recipeSummarySelect } from '../recipe/queries';

export const selectWeekPlanMeal = Prisma.validator<Prisma.WeekPlanMealFindManyArgs>()({
	select: {
		id: true,
		day: true,
		slot: true,
		size: true,
		name: true,
		tags: tagWithCategory,
		recipe: recipeSummarySelect
	}
});

export const selectWeekPlan = Prisma.validator<Prisma.WeekPlanFindManyArgs>()({
	select: {
		week: true,
		year: true,
		meals: selectWeekPlanMeal
	}
});

export const selectWeekPlanSummary = Prisma.validator<Prisma.WeekPlanFindManyArgs>()({
	select: {
		week: true,
		year: true
	}
});
