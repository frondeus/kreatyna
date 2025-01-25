export { matchers } from "./matchers.js";

export const nodes = [
  () => import("./nodes/0"),
  () => import("./nodes/1"),
  () => import("./nodes/2"),
  () => import("./nodes/3"),
  () => import("./nodes/4"),
  () => import("./nodes/5"),
  () => import("./nodes/6"),
  () => import("./nodes/7"),
  () => import("./nodes/8"),
  () => import("./nodes/9"),
  () => import("./nodes/10"),
  () => import("./nodes/11"),
  () => import("./nodes/12"),
  () => import("./nodes/13"),
  () => import("./nodes/14"),
  () => import("./nodes/15"),
  () => import("./nodes/16"),
  () => import("./nodes/17"),
  () => import("./nodes/18"),
  () => import("./nodes/19"),
  () => import("./nodes/20"),
  () => import("./nodes/21"),
  () => import("./nodes/22"),
  () => import("./nodes/23"),
  () => import("./nodes/24"),
];

export const server_loads = [0, 2, 4, 5];

export const dictionary = {
  "/": [~6],
  "/account": [~7],
  "/backoffice/import-raw": [~8, [2]],
  "/browse": [~9],
  "/browse/raw": [~10],
  "/browse/raw/create": [~12],
  "/browse/raw/[id]": [~11, [], [3]],
  "/browse/recipe": [~13],
  "/browse/recipe/create": [~15],
  "/browse/recipe/[id]": [~14],
  "/browse/tag": [~16],
  "/browse/tag/create": [~17],
  "/login": [~18],
  "/plan": [~19, [4]],
  "/plan/today": [~21, [4]],
  "/plan/[year]/[week]": [~20, [4]],
  "/shop": [22, [5]],
  "/shop/add": [23, [5]],
  "/signup": [~24],
};

export const hooks = {
  handleError: ({ error }) => {
    console.error(error);
  },
};

export { default as root } from "../root.svelte";
