import { invoke } from "@tauri-apps/api/core";

export async function getShoppingLists(): Promise<any[]> {
    return await invoke('get_documents', { kind: 'shoppingList' });
}

export async function createShoppingList(doc: any): Promise<any> {
    return await invoke('create_document', { newDoc: doc });
}