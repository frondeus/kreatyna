import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals: { session, container } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const shopping = container.shoppingListService;
	const items = shopping.getShoppingListItems();

	return {
		session,
		items
	};
};
