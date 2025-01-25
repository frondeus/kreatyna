import type { Readable } from "svelte/motion";
import type { Page } from "@sveltejs/kit";
import * as svelte from "svelte";

export interface Context {
  page: Readable<Page>;
}

export function getContext(): Context {
  return svelte.getContext<Context>("page");
}

export function setContext(context: Context) {
  svelte.setContext("page", context);
}
