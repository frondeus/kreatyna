import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createContext } from "react";
import {
  AddShoppingListItem,
  DocumentChangeEntry,
  DocumentId,
  EventNotification,
  ShoppingList,
} from "./types";

async function getShoppingLists(): Promise<ShoppingList[]> {
  return await invoke("get_documents", { kind: "shoppingList" });
}

async function getItemsAndSuggestions(
  query: string,
): Promise<AddShoppingListItem[]> {
  return await invoke("get_items_and_suggestions", { query });
}

async function updateDocument(
  id: DocumentId,
  changes: DocumentChangeEntry[],
): Promise<void> {
  return await invoke("update_document", { id, changes });
}

function subscribeToEvents(cbk: (msg: EventNotification) => void): () => void {
  const unsub = listen<EventNotification>("event", (e) => {
    cbk(e.payload);
  });

  return () => {
    unsub.then((u) => u());
  };
}

export interface API {
  getShoppingLists: () => Promise<ShoppingList[]>;
  getItemsAndSuggestions: (query: string) => Promise<AddShoppingListItem[]>;
  subscribeToEvents: (cbk: (msg: EventNotification) => void) => () => void;
  updateDocument: (
    id: DocumentId,
    changes: DocumentChangeEntry[],
  ) => Promise<void>;
}

export const api: API = {
  getShoppingLists,
  getItemsAndSuggestions,
  subscribeToEvents,
  updateDocument,
};

export default api;

export const APIContext = createContext<API>(api);
