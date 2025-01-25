export type DocumentId = number;

export interface DocumentMeta {
  id: DocumentId;
  title: string;
}

export interface ShoppingList extends DocumentMeta {
  kind: "shoppingList";
  items: ShoppingListItem[];
}

export type Document = ShoppingList;

export interface ShoppingListItem {
  id: string;
  name: string;
  amount?: number;
  unit?: string;
  state: ShoppingListItemState;
}

export type ShoppingListItemProposal = Omit<ShoppingListItem, "id" | "state"> & {
  state?: ShoppingListItemState;
};

export enum ShoppingListItemState {
  Todo = "TODO",
  InCart = "IN_CART",
  Bought = "BOUGHT",
  NotBuying = "NOT_BUYING",
}
