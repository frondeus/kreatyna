import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ depends, locals: { session, container } }) => {
	depends('data:now');

	if (!session) {
		throw redirect(303, '/login');
	}

	const shopping = container.shoppingListService;
	const items = shopping.getShoppingListItems();

	// console.log('Fetch', { items});
	// console.log('fetch', items[0].state);

	return {
		session,
		items
	};
};
