import type { Prisma } from '@prisma/client';
import type {
	selectWeekPlan,
	selectWeekPlanMeal,
	selectWeekPlanRecipes,
	selectWeekPlanSummary
} from './queries';

export type WeekPlanMealWithId = Prisma.WeekPlanMealGetPayload<typeof selectWeekPlanMeal>;
export type WeekPlanMeal = WeekPlanMealWithId;
export type WeekPlanMealWithoutId = Omit<WeekPlanMealWithId, 'id'>;

export type WeekPlanMealWithRecipe = Prisma.WeekPlanMealGetPayload<typeof selectWeekPlanRecipes>;

export type WeekPlanWithId = Prisma.WeekPlanGetPayload<typeof selectWeekPlan>;
export type WeekPlan = Omit<WeekPlanWithId, 'meals'> & {
	meals: WeekPlanMeal[];
};
export type WeekPlanCreate = Omit<WeekPlanWithId, 'meals'> & {
	meals: WeekPlanMealWithoutId[];
};

export type WeekPlanSummary = Prisma.WeekPlanGetPayload<typeof selectWeekPlanSummary>;

export interface Size {
	index: number;
	size?: number;
}
export interface Day {
	name: string;
}

export const daysInWeek: Day[] = [
	{
		name: 'Sunday'
	},
	{
		name: 'Monday'
	},
	{
		name: 'Tuesday'
	},
	{
		name: 'Wednesday'
	},
	{
		name: 'Thursday'
	},
	{
		name: 'Friday'
	},
	{
		name: 'Saturday'
	}
];
