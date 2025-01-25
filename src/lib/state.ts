import { create } from "zustand";
import { use } from "react-use-polyfill";
import { ShoppingList, ShoppingListItem } from "./types";

export type Route = "shop";

export interface RouteState {
    route: Route;
    setRoute: (route: Route) => void;
}

export const useRoute = create<RouteState>((set) => ({
    route: "shop",
    setRoute: (route: Route) => set({ route }),
}));

export interface UsePromise<T> {
    promise?: Promise<T>;
    refresh (): void;
    fetch(call: () => Promise<T>): T;
}

function createUsePromise<T>( ) {
    return create<UsePromise<T>>((set, get) => ({
        refresh: () => set({ promise: undefined }),
        fetch: (call: () => Promise<T>) => {
            const state = get();
            let promise = state.promise;
            if (!promise) {
                promise = call();
                set({ promise });
            }
            return use(promise)
        }
    }));
}

export const useShop = createUsePromise<ShoppingList[]>();
export const useSuggestions = createUsePromise<ShoppingListItem[]>();
