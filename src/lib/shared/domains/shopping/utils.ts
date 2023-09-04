import type { ShoppingListItem } from './models';

export const sortByState: (a: ShoppingListItem, b: ShoppingListItem) => number = (a, b) => {
	if (a.state !== b.state) {
		if (a.state === 'TODO') return -1;
		if (b.state === 'TODO') return 1;

		if (a.state === 'NOT_BUING') return -1;
		if (b.state === 'NOT_BUING') return 1;

		if (a.state === 'IN_CART') return -1;
		if (b.state === 'IN_CART') return 1;
	}

	return a.order - b.order;
};
