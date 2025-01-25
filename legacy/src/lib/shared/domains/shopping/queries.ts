import { Prisma } from "@prisma/client";
import { ingredientsWithRecipes } from "../recipe/queries";

export const selectShoppingListCategoryForItem =
  Prisma.validator<Prisma.ShoppingListCategoryFindManyArgs>()({
    select: {
      id: true,
      name: true,
      icon: true,
      order: true,
    },
  });

export const selectShoppingListItemsWithoutCategory =
  Prisma.validator<Prisma.ShoppingListItemFindManyArgs>()({
    select: {
      id: true,
      name: true,
      amount: true,
      unit: true,
      state: true,
      order: true,
      ingredients: ingredientsWithRecipes,
    },
  });

export const selectShoppingListItems =
  Prisma.validator<Prisma.ShoppingListItemFindManyArgs>()({
    select: {
      ...selectShoppingListItemsWithoutCategory.select,
      category: selectShoppingListCategoryForItem,
    },
  });

export const selectShoppingListCategories =
  Prisma.validator<Prisma.ShoppingListCategoryFindManyArgs>()({
    select: {
      ...selectShoppingListCategoryForItem.select,
      items: selectShoppingListItemsWithoutCategory,
    },
  });
