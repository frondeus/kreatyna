import { notifications } from "@mantine/notifications";
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
  try {
    return await invoke("get_documents", { kind: "shoppingList" });
  } catch (e) {
    console.error("Failed to load shopping lists", e);
    notifications.show({
      title: "Failed to load shopping lists",
      message: e as string,
      color: "red",
    });
    return [];
  }
}

async function getItemsAndSuggestions(
  query: string,
): Promise<AddShoppingListItem[]> {
  try {
    return await invoke("get_items_and_suggestions", { query });
  } catch (e) {
    console.error("Failed to load items and suggestions", e);
    notifications.show({
      title: "Failed to load items and suggestions",
      message: e as string,
      color: "red",
    });
    return [];
  }
}

async function updateDocument(
  id: DocumentId,
  changes: DocumentChangeEntry[],
): Promise<void> {
  try {
    return await invoke("update_document", { id, changes });
  } catch (e) {
    console.error("Failed to update document", e);
    notifications.show({
      title: "Failed to update document",
      message: e as string,
      color: "red",
    });
  }
}

/** eslint-disable-next-line @typescript-eslint/no-explicit-any */
async function log(msg: string, args: any[]) {
  await invoke("log", { msg, args });
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
  log: (msg: string, args: any[]) => Promise<void>;
}

export const api: API = {
  getShoppingLists,
  getItemsAndSuggestions,
  subscribeToEvents,
  updateDocument,
  log,
};

export default api;

export const APIContext = createContext<API>(api);
