import type { ShoppingListItem, StoredShoppingListItem } from '$shared/domains/shopping/models';

export class ShoppingListItemMap {
	toDomain(item: StoredShoppingListItem): ShoppingListItem {
		return {
			id: item.id,
			name: item.name,
			amount: item.amount.toNumber(),
			unit: item.unit,
			order: item.order,
			state: item.state
		};
	}
}
