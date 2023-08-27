import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { MealForDay } from '$shared/domains/plan/models';

// This script is released to the public domain and may be used, modified and
// distributed without restrictions. Attribution not necessary but appreciated.
// Source: https://weeknumber.com/how-to/javascript

// Returns the ISO week of the date.
Date.prototype.getWeek = function () {
	const date = new Date(this.getTime());
	date.setHours(0, 0, 0, 0);
	// Thursday in current week decides the year.
	date.setDate(date.getDate() + 3 - ((date.getDay() + 6) % 7));
	// January 4 is always in week 1.
	const week1 = new Date(date.getFullYear(), 0, 4);
	// Adjust to Thursday in week 1 and count number of weeks from date to week1.
	return (
		1 +
		Math.round(((date.getTime() - week1.getTime()) / 86400000 - 3 + ((week1.getDay() + 6) % 7)) / 7)
	);
};

export const load: PageServerLoad = async ({ locals: { session, services, maps } }) => {
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

	let weekPlan = await services.weekPlan.getWeekPlan(year, week);

	if (weekPlan === null) {
		weekPlan = await services.weekPlan.generatePopulateAndSaveWeekPlan(year, week);
	}

	const meals: MealForDay[] = weekPlan.meals.map((meal) => maps.weekPlanMeal.toDomain(meal));

	return { session, meals };
};
