// @ts-nocheck
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = async () => {
	// const session = await getSession();

	// if (!session) {
	// 	throw redirect(303, '/login');
	// }

	throw redirect(303, '/plan');
	// return {};
};
;null as any as PageServerLoad;