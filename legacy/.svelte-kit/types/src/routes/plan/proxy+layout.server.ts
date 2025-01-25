// @ts-nocheck
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load = async ({ locals: { session, container } }: Parameters<LayoutServerLoad>[0]) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const date = new Date();
	const year = date.getFullYear();
	const dayOfWeek = date.getDay();
	let week = new Date().getWeek();

	let isWeekend = false;
	/// If its saturday or sunday, get the next week
	if (dayOfWeek === 0 || dayOfWeek === 6) {
		week += 1;
		isWeekend = true;
	}

	const service = container.weekPlanService;

	const plans = service.getWeekPlanSummaries();

	return {
		plans,
		current: {
			year,
			week,
			isWeekend
		}
	};
};
