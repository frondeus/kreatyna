import type { PageServerLoad } from './$types';
import type { WeekPlanWithId } from '$shared/domains/plan/models';
import { WeekPlanService } from '$server/domains/plan/service';

export const load: PageServerLoad = async ({ params, locals: { container } }) => {
	const year = Number(params.year);
	const week = Number(params.week);

	const service = container.resolve(WeekPlanService);
	const weekPlan: WeekPlanWithId | null = await service.getWeekPlan(year, week);

	if (weekPlan === null) {
		const createdWeekPlan = await service.generateAndSaveWeekPlan(year, week);
		return { weekPlan: createdWeekPlan };
	}

	return { weekPlan };
};
