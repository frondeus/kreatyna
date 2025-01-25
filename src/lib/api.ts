import { invoke } from "@tauri-apps/api/core";
import { createContext } from "react";
import { EventNotification, ShoppingList, ShoppingListSuggestion } from "./types";
import { listen } from "@tauri-apps/api/event";

async function getShoppingLists(): Promise<ShoppingList[]> {
  return await invoke("get_documents", { kind: "shoppingList" });
}

async function getItemSuggestions(query: string): Promise<ShoppingListSuggestion[]> {
  return await invoke("get_item_suggestions", { query });
}

function subscribeToEvents(cbk: (msg: EventNotification) => void): (() => void) {
  const unsub = listen<EventNotification>("event", (e) => {
    cbk(e.payload);
  });

  return () => {
    unsub.then((u) => u());
  };
}

export interface API {
  getShoppingLists: () => Promise<ShoppingList[]>;
  getItemSuggestions: (query: string) => Promise<ShoppingListSuggestion[]>;
  subscribeToEvents: (cbk: (msg: EventNotification) => void) => (() => void);
}

export const api: API = {
  getShoppingLists,
  getItemSuggestions,
  subscribeToEvents,
};

export default api;

export const APIContext = createContext<API>(api);
