import { invoke } from "@tauri-apps/api/core";
import { DocumentId, ShoppingList } from "./types";
import { NewDocument } from "./schema";
import { createContext } from "react";

async function getShoppingLists(): Promise<ShoppingList[]> {
    return await invoke('get_documents', { kind: 'shoppingList' });
}

async function createShoppingList(doc: NewDocument): Promise<DocumentId> {
    return await invoke('create_document', { newDoc: doc });
}

export interface API {
    getShoppingLists: () => Promise<ShoppingList[]>,
    createShoppingList: (doc: NewDocument) => Promise<DocumentId>
};

export const api: API = {
    getShoppingLists,
    createShoppingList
};

export default api;

export const APIContext = createContext<API>(api);

