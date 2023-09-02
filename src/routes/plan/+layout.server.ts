import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { WeekPlanSummary } from '$shared/domains/plan/models';

export const load: LayoutServerLoad = async ({ locals: { session, container } }) => {
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

	const service = container.weekPlanService;

	const plans: WeekPlanSummary[] = await service.getWeekPlanSummaries();

	return {
		plans,
		current: {
			year,
			week
		}
	};
};
