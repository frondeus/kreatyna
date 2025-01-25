import { create } from "zustand";

export type Route =
  | "shop"
  | "raw-ingredient-list"
  | "recipe-list"
  | {
      type: "raw-ingredient";
      id: string;
    };

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
