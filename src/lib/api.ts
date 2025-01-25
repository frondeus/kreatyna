import { notifications } from "@mantine/notifications";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createContext, useContext } from "react";
import {
  AddShoppingListItem,
  Document,
  DocumentChangeEntry,
  DocumentId,
  DocumentKind,
  EventNotification,
} from "./types";

async function getDocuments(kind: DocumentKind): Promise<Document[]> {
  try {
    return await invoke("get_documents", { kind });
  } catch (e) {
    console.error("Failed to load documents", e);
    notifications.show({
      title: "Failed to load documents",
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

async function newDocument(kind: DocumentKind): Promise<DocumentId> {
  return await invoke("new_document", { kind });
}

async function getDocument(id: DocumentId): Promise<Document> {
  return await invoke("get_document", { id });
}

async function finishShopping(id: DocumentId): Promise<void> {
  return await invoke("finish_shopping", { id });
}

/** eslint-disable-next-line @typescript-eslint/no-explicit-any */
function subscribeToEvents(cbk: (msg: EventNotification) => void): () => void {
  const unsub = listen<EventNotification>("event", (e) => {
    cbk(e.payload);
  });

  return () => {
    unsub.then((u) => u());
  };
}

function subscribeToDocumentEvents(
  id: DocumentId,
  cbk: (msg: EventNotification) => void,
): () => void {
  return subscribeToEvents((msg) => {
    if (msg.id !== id) return;
    cbk(msg);
  });
}

export interface API {
  getItemsAndSuggestions: (query: string) => Promise<AddShoppingListItem[]>;
  subscribeToEvents: (cbk: (msg: EventNotification) => void) => () => void;
  subscribeToDocumentEvents: (
    id: DocumentId,
    cbk: (msg: EventNotification) => void,
  ) => () => void;
  updateDocument: (
    id: DocumentId,
    changes: DocumentChangeEntry[],
  ) => Promise<void>;
  finishShopping: (id: DocumentId) => Promise<void>;
  getDocuments: (kind: DocumentKind) => Promise<Document[]>;
  getDocument: (id: DocumentId) => Promise<Document>;
  newDocument: (kind: DocumentKind) => Promise<DocumentId>;
}

export const api: API = {
  getItemsAndSuggestions,
  subscribeToEvents,
  subscribeToDocumentEvents,
  updateDocument,
  finishShopping,
  getDocuments,
  getDocument,
  newDocument,
};

export default api;

export const APIContext = createContext<API>(api);

export const useAPI = () => useContext(APIContext);
