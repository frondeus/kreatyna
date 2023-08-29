import type { WeekPlan } from '$shared/domains/plan/models';
import type { PrismaClient } from '@prisma/client';
import type { WeekPlanMap } from './map';
import { weekPlan } from '$shared/domains/plan/queries';
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

	async getWeekPlan(year: number, week: number): Promise<WeekPlan | null> {
		return await this.prisma.weekPlan.findUnique({
			...weekPlan,
			where: {
				year_week: {
					year: year,
					week: week
				}
			}
		});
	}

	async createWeekPlan(weekPlan: WeekPlan): Promise<WeekPlan> {
		const createInput = this.weekPlanMap.toCreateInput(weekPlan);

		await this.prisma.weekPlan.create({
			data: createInput
		});

		return weekPlan;
	}

	async generatePopulateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.weekPlanGenerator.generateWeekPlan(year, week);
		const populatedPlan = await this.weekPlanPopulator.populateWeekPlan(weekPlan);

		return await this.createWeekPlan(populatedPlan);
	}
}
