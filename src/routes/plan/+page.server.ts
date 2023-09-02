import type { PageServerLoad } from './$types';
import type { WeekPlanWithId } from '$shared/domains/plan/models';

export const load: PageServerLoad = async ({ locals: { container }, parent }) => {
	const {
		current: { year, week }
	} = await parent();

	const service = container.weekPlanService;
	const weekPlan: WeekPlanWithId | null = await service.getWeekPlan(year, week);

	if (weekPlan === null) {
		const createdWeekPlan = await service.generatePopulateAndSaveWeekPlan(year, week);
		return { weekPlan: createdWeekPlan };
	}

	return { weekPlan };
};
