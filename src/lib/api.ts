import { invoke } from "@tauri-apps/api/core";
import { createContext } from "react";
import { NewDocument } from "./schema";
import { DocumentId, ShoppingList, ShoppingListItem, ShoppingListItemProposal } from "./types";

async function getShoppingLists(): Promise<ShoppingList[]> {
  return await invoke("get_documents", { kind: "shoppingList" });
}

async function createShoppingList(doc: NewDocument): Promise<DocumentId> {
  return await invoke("create_document", { newDoc: doc });
}

async function getItemSuggestions(_query: string): Promise<ShoppingListItem[]> {
  return new Promise(() => {});
}

export interface API {
  getShoppingLists: () => Promise<ShoppingList[]>;
  createShoppingList: (doc: NewDocument) => Promise<DocumentId>;
  getItemSuggestions: (query: string) => Promise<ShoppingListItemProposal[]>;
}

export const api: API = {
  getShoppingLists,
  createShoppingList,
  getItemSuggestions,
};

export default api;

export const APIContext = createContext<API>(api);
