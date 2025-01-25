import { create } from "zustand";
import * as api from "./api";
import { use } from "react-use-polyfill";

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
    fetch(): T;
}

function createUsePromise<T>(
    fetch: () => Promise<T>
) {
    return create<UsePromise<T>>((set, get) => ({
        refresh: () => set({ promise: undefined }),
        fetch: () => {
            let promise = get().promise;
            if (!promise) {
                promise = fetch();
                set({ promise });
            }
            return use(promise)
        }
    }));
}

export const useShop = createUsePromise(api.getShoppingLists);
