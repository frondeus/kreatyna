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
  amount: number;
  unit?: string;
  state: ShoppingListItemState;
}

export type ShoppingListSuggestion = Omit<ShoppingListItem, "id" | "state"> & {
  id?: string;
  state?: ShoppingListItemState;
};

export enum ShoppingListItemState {
  Todo = "TODO",
  InCart = "IN_CART",
  Bought = "BOUGHT",
  NotBuying = "NOT_BUYING",
}


interface DocumentCreated {
  kind: "created";
  id: DocumentId;
}

interface DocumentReplaced {
  kind: "replaced";
  id: DocumentId;
}

interface DocumentUpdated {
  kind: "updated";
  id: DocumentId;
  changes: EventNotificationChange[];
}

interface DocumentDeleted {
  kind: "deleted";
  id: DocumentId;
}

export type EventNotification =
  | DocumentCreated
  | DocumentReplaced
  | DocumentUpdated
  | DocumentDeleted;

interface ChangeReplace {
  kind: "replace";
  data: unknown;
}

interface ChangeReplaceWithUndefined {
  kind: "replaceWithUndefined";
}

interface ChangeMove {
  kind: "move";
  from: number;
  to: number;
}

interface ChangeRemove {
  kind: "remove";
  index: number;
}

interface ChangeInsert {
  kind: "insert";
  position?: number;
  value: unknown;
}

export type EventNotificationChange =
  | ChangeReplace
  | ChangeReplaceWithUndefined
  | ChangeMove
  | ChangeRemove
  | ChangeInsert;
