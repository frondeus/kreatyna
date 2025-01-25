// @ts-nocheck
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = async ({
  depends,
  locals: { session, container },
}: Parameters<LayoutServerLoad>[0]) => {
  depends("data:shopping-items");

  if (!session) {
    throw redirect(303, "/login");
  }

  const shopping = container.shoppingListService;
  const items = shopping.getShoppingListItems();
  const categories = shopping.getShoppingListCategories();

  return {
    items,
    categories,
  };
};
