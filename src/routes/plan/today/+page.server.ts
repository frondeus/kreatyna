import type { PageServerLoad } from './$types';
import type { WeekPlanWithId } from '$shared/domains/plan/models';

export const load: PageServerLoad = async ({ locals: { container }, parent }) => {
	const {
		current: { year, week: maybeWeek, isWeekend }
	} = await parent();

	let week = maybeWeek;
	if (isWeekend) {
		week -= 1;
	}

	const service = container.weekPlanService;
	const weekPlan: WeekPlanWithId | null = await service.getWeekPlan(year, week);

	if (weekPlan === null) {
		const createdWeekPlan = await service.generateAndSaveWeekPlan(year, week);
		return { weekPlan: createdWeekPlan };
	}

	return { weekPlan };
};
