import { error, json, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { WeekPlanService } from '$server/domains/plan/service';
import { z } from 'zod';
import type { WeekPlanWithId } from '$shared/domains/plan/models';

const BodySchema = z.object({
	week: z.number(),
	year: z.number()
});

export const POST = (async ({ request, locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const body = await request.json();
	const validatedBody = BodySchema.safeParse(body);
	if (validatedBody.success === false) {
		throw error(400, validatedBody.error.message);
	}

	const { week, year } = validatedBody.data;
	const service = container.resolve(WeekPlanService);

	const weekPlan: WeekPlanWithId | null = await service.getWeekPlan(year, week);

	if (weekPlan === null) {
		throw error(404, 'Week plan not found');
	}

	await service.populateAndUpdateWeekPlan(weekPlan);

	return json('ok');
}) satisfies RequestHandler;
