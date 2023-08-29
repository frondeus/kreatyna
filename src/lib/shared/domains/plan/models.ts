import type { Prisma } from '@prisma/client';
import type { selectWeekPlan, selectWeekPlanMeal } from './queries';

export type WeekPlanMealWithId = Prisma.WeekPlanMealGetPayload<typeof selectWeekPlanMeal>;
export type WeekPlanMeal = Omit<WeekPlanMealWithId, 'id'>;

export type WeekPlanWithId = Prisma.WeekPlanGetPayload<typeof selectWeekPlan>;
export type WeekPlan = Omit<WeekPlanWithId, 'meals'> & {
	meals: WeekPlanMeal[];
};

export interface Size {
	index: number;
	size?: number;
}
