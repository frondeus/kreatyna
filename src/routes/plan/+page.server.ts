import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { WeekPlan } from '$shared/domains/plan/models';
import { WeekPlanService } from '$server/domains/plan/service';

export const load: PageServerLoad = async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}
	const date = new Date();
	const year = date.getFullYear();
	const dayOfWeek = date.getDay();
	let week = new Date().getWeek();

	/// If its saturday or sunday, get the next week
	if (dayOfWeek === 0 || dayOfWeek === 6) {
		week += 1;
	}

	const service = container.resolve(WeekPlanService);
	let weekPlan: WeekPlan | null = await service.getWeekPlan(year, week);

	if (weekPlan === null) {
		weekPlan = await service.generatePopulateAndSaveWeekPlan(year, week);
	}

	return { session, weekPlan };
};
