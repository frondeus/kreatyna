// @ts-nocheck
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load = async () => {
	throw redirect(303, '/plan/today');
};
;null as any as PageServerLoad;