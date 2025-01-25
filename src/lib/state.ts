import { useEffect } from "react";
import { use } from "react-use-polyfill";
import { create } from "zustand";
import { AddShoppingListItem, RawIngredient, ShoppingList } from "./types";

export type Route = "shop" | "raw-ingredient-list";

export interface RouteState {
  route: Route;
  setRoute: (route: Route) => void;
  goto(route: Route): () => void;
}

export const useRoute = create<RouteState>((set) => ({
  route: "shop",
  setRoute: (route: Route) => set({ route }),
  goto(route: Route) {
    return () => set({ route });
  },
}));

export interface UsePromise<T> {
  promise?: Promise<T>;
  refresh(): void;
  fetch(call: () => Promise<T>): T;
  fetchWithArgs<A>(call: (args: A) => Promise<T>, args: A): T;
}

function createUsePromise<T>() {
  return create<UsePromise<T>>((set, get) => ({
    refresh: () => set({ promise: undefined }),
    fetchWithArgs<A>(call: (args: A) => Promise<T>, args: A) {
      const state = get();
      if (args !== undefined) {
        useEffect(() => {
          const promise = call(args);
          set({ promise });
        }, [args]);
      }
      let promise = state.promise;
      if (!promise) {
        promise = call(args);
        set({ promise });
      }
      return use(promise);
    },
    fetch(call: () => Promise<T>) {
      return get().fetchWithArgs(() => call(), undefined);
    },
  }));
}

export const useShop = createUsePromise<ShoppingList[]>();
export const useItemsOrSuggestions = createUsePromise<AddShoppingListItem[]>();
export const useRawIngredients = createUsePromise<RawIngredient[]>();
