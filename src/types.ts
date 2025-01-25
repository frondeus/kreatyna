export type DocumentId = number;

export interface DocumentMeta {
    id: DocumentId;
    title: string;
}

export interface ShoppingList extends DocumentMeta {
    kind: 'shoppingList';
    items: ShoppingListItem[];
}

export type Document = ShoppingList;

export interface ShoppingListItem {
    name: string;
    amount?: string;
    hasScale?: boolean;
}