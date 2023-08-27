import { Prisma } from '@prisma/client';
import { tagWithCategory } from '../tags/queries';
import { recipeSummarySelect } from '../recipe/queries';

export const weekPlanMeal = Prisma.validator<Prisma.WeekPlanMealFindManyArgs>()({
	select: {
		day: true,
		slot: true,
		size: true,
		name: true,
		tags: tagWithCategory,
		recipe: recipeSummarySelect
	}
});

export const weekPlan = Prisma.validator<Prisma.WeekPlanFindManyArgs>()({
	select: {
		week: true,
		year: true,
		meals: weekPlanMeal
	}
});
