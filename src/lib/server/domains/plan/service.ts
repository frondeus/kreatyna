import type {
	WeekPlanWithId,
	WeekPlan,
	WeekPlanCreate,
	WeekPlanSummary
} from '$shared/domains/plan/models';
import type { PrismaClient } from '@prisma/client';
import type { WeekPlanMap } from './map';
import { selectWeekPlan, selectWeekPlanSummary } from '$shared/domains/plan/queries';
import { Lifecycle, inject, injectable, scoped } from 'tsyringe';
import type { WeekPlanGenerator } from './generator';
import type { WeekPlanPopulatorFactory } from './populator';

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanService {
	constructor(
		@inject('prisma') readonly prisma: PrismaClient,
		@inject('weekPlanMap') readonly weekPlanMap: WeekPlanMap,
		@inject('weekPlanGenerator') readonly weekPlanGenerator: WeekPlanGenerator,
		@inject('weekPlanPopulator') readonly weekPlanPopulator: WeekPlanPopulatorFactory
	) {}

	async getWeekPlanSummaries(): Promise<WeekPlanSummary[]> {
		return await this.prisma.weekPlan.findMany({
			...selectWeekPlanSummary,
			orderBy: [
				{
					year: 'desc'
				},
				{
					week: 'desc'
				}
			],
			take: 5
		});
	}

	async getWeekPlan(year: number, week: number): Promise<WeekPlanWithId | null> {
		return await this.prisma.weekPlan.findUnique({
			...selectWeekPlan,
			where: {
				year_week: {
					year: year,
					week: week
				}
			}
		});
	}

	async createWeekPlan(weekPlan: WeekPlanCreate): Promise<WeekPlan> {
		const createInput = this.weekPlanMap.toCreateInput(weekPlan);

		return await this.prisma.weekPlan.create({
			data: createInput,
			...selectWeekPlan
		});
	}

	async updateWeekPlan(weekPlan: WeekPlanWithId): Promise<WeekPlanWithId> {
		const updateInput = this.weekPlanMap.toUpdateInput(weekPlan);
		await this.prisma.weekPlan.update({
			data: updateInput,
			where: {
				year_week: {
					year: weekPlan.year,
					week: weekPlan.week
				}
			}
		});
		return weekPlan;
	}

	async unlinkMeals(meals: number[], year: number, week: number): Promise<void> {
		await this.prisma.weekPlanMeal.updateMany({
			data: {
				recipe_id: null
			},
			where: {
				id: {
					in: meals
				},
				weekPlan: {
					year,
					week
				}
			}
		});
	}

	async generateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.weekPlanGenerator.generateWeekPlan(year, week);
		return await this.createWeekPlan(weekPlan);
	}

	async generatePopulateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.weekPlanGenerator.generateWeekPlan(year, week);
		const populatedPlan = await this.weekPlanPopulator.populateWeekPlan(weekPlan);

		return await this.createWeekPlan(populatedPlan);
	}

	async populateAndUpdateWeekPlan(weekPlan: WeekPlanWithId): Promise<WeekPlanWithId> {
		const populatedPlan = await this.weekPlanPopulator.populateWeekPlan(weekPlan);

		return await this.updateWeekPlan(populatedPlan);
	}
}
