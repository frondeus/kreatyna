import { Prisma } from '@prisma/client';
import type { RecipeSummary } from '../recipe/models';

export const weekPlanMeal = Prisma.validator<Prisma.WeekPlanMealFindManyArgs>()({
	select: {
		day: true,
		slot: true,
		size: true,
		name: true,
		tags: {
			select: {
				id: true,
				name: true,
				category: {
					select: {
						name: true
					}
				}
			}
		},
		recipe: {
			select: {
				id: true,
				name: true,
				tags: {
					select: {
						id: true,
						name: true,
						category: {
							select: {
								name: true
							}
						}
					}
				}
			}
		}
	}
});

export type WeekPlanMeal = Prisma.WeekPlanMealGetPayload<typeof weekPlanMeal>;

export const weekPlan = Prisma.validator<Prisma.WeekPlanFindManyArgs>()({
	select: {
		week: true,
		year: true,
		meals: weekPlanMeal
	}
});

export type WeekPlan = Prisma.WeekPlanGetPayload<typeof weekPlan>;

export interface Size {
	index: number;
	size?: number;
}

export interface MealForDay {
	name: string;
	day: number;
	slot: Size;

	tags: number[];
	recipe?: RecipeSummary;
}
