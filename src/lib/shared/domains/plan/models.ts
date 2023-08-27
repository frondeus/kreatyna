import type { Prisma } from '@prisma/client';
import type { weekPlan, weekPlanMeal } from './queries';

export type WeekPlanMeal = Prisma.WeekPlanMealGetPayload<typeof weekPlanMeal>;
export type WeekPlan = Prisma.WeekPlanGetPayload<typeof weekPlan>;

export interface Size {
	index: number;
	size?: number;
}
