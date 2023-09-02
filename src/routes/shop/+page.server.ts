import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { ShoppingListItem } from '$shared/domains/shopping/models';

const sortByState: (a: ShoppingListItem, b: ShoppingListItem) => number = (a, b) => {
	if (a.state !== b.state) {
		if (a.state === 'TODO') return -1;
		if (b.state === 'TODO') return 1;

		if (a.state === 'IN_CART') return -1;
		if (b.state === 'IN_CART') return 1;
	}

	return a.order - b.order;
};

export const load: PageServerLoad = async ({ locals: { session } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const items: ShoppingListItem[] = [
		// from 0..20 to array of items
		...Array.from(Array(20).keys()).map((i) => ({
			id: i,
			name: `Item ${i}`,
			amount: (i + 1) * 100,
			unit: 'g',
			order: i,
			state: i % 5 == 0 ? ('IN_CART' as const) : ('TODO' as const)
		}))
	].sort(sortByState);

	return {
		session,
		items
	};
};
