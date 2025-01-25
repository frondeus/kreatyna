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
  id: string;
};

export type AddShoppingListItem =
  | (ShoppingListItem & { kind: "item" })
  | (ShoppingListSuggestion & { kind: "suggestion" });

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

interface ChangeReplaced {
  kind: "replace";
  data: unknown;
}

interface ChangeReplacedWithUndefined {
  kind: "replaceWithUndefined";
}

interface ChangeMoved {
  kind: "move";
  from: number;
  to: number;
}

interface ChangeRemoved {
  kind: "remove";
  index: number;
}

interface ChangeInserted {
  kind: "insert";
  position?: number;
  value: unknown;
}

export type EventNotificationChange =
  | ChangeReplaced
  | ChangeReplacedWithUndefined
  | ChangeMoved
  | ChangeRemoved
  | ChangeInserted;

export interface ChangeReplace {
  kind: "replace";
  data: unknown;
}

export interface ChangeReplaceWithUndefined {
  kind: "replaceWithUndefined";
}

interface ChangeMove {
  kind: "move";
  key: string;
  before?: string;
}

export interface ChangeRemove {
  kind: "remove";
  key: string;
}

export interface ChangeInsert {
  kind: "insert";
  before?: string;
  value: unknown;
  key: string;
}

export type DocumentChangeEntry = [string, DocumentChange];

export type DocumentChange =
  | ChangeReplace
  | ChangeReplaceWithUndefined
  | ChangeMove
  | ChangeRemove
  | ChangeInsert;
