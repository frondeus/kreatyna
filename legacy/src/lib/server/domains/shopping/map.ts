import type { GroupedRecipeIngredient } from "$shared/domains/recipe/models";
import type {
  ShoppingListItem,
  StoredShoppingListItem,
} from "$shared/domains/shopping/models";
import type {
  Category,
  CreateShoppingListItemWithCategory,
  UpdateShoppingListItem,
  UpdateShoppingListItemWithCategory,
} from "$shared/domains/shopping/schema";
import type { Prisma } from "@prisma/client";
import { v4 } from "uuid";
import type { RecipeIngredientMap } from "../recipe/map";

export class ShoppingListItemMap {
  constructor(readonly ingredientMap: RecipeIngredientMap) {}

  toCreateInputFromRecipeIngerdient(
    ingredient: GroupedRecipeIngredient,
    category: Category,
  ): Prisma.ShoppingListItemCreateInput {
    return {
      id: v4(),
      name: ingredient.raw?.name ?? ingredient.name,
      amount: ingredient.amount.toNumber(),
      unit: ingredient.unit,
      state: "TODO",
      category: {
        connectOrCreate: {
          where: {
            id: category.id,
          },
          create: {
            id: category.id,
            name: category.name,
            icon: category.emoji,
          },
        },
      },
      ingredients: {
        connect: ingredient.ids.map((id) => ({ id })),
      },
    };
  }

  toUpdateInputFromRecipeIngerdient(
    ingredient: GroupedRecipeIngredient,
    category: Category,
    clear: boolean,
  ): Prisma.ShoppingListItemUpdateInput {
    return {
      name: ingredient.raw?.name ?? ingredient.name,
      amount: clear
        ? ingredient.amount.toNumber()
        : {
            increment: ingredient.amount.toNumber(),
          },
      unit: ingredient.unit,
      state: "TODO",
      category: {
        connectOrCreate: {
          where: {
            id: category.id,
          },
          create: {
            id: category.id,
            name: category.name,
            icon: category.emoji,
          },
        },
      },
      ingredients: clear
        ? {
            set: ingredient.ids.map((id) => ({ id })),
          }
        : {
            connect: ingredient.ids.map((id) => ({ id })),
          },
    };
  }

  toDomain(item: StoredShoppingListItem): ShoppingListItem {
    return {
      id: item.id,
      name: item.name,
      amount: item.amount.toNumber(),
      unit: item.unit,
      order: item.order,
      state: item.state,
      ingredients: item.ingredients.map((i) =>
        this.ingredientMap.toDomainWithRecipe(i),
      ),
      category: item.category,
    };
  }

  toCreateInput(
    item: CreateShoppingListItemWithCategory,
  ): Prisma.ShoppingListItemCreateInput {
    return {
      id: item.id,
      name: item.name,
      state: item.state,
      amount: item.amount,
      unit: item.unit ?? null,
      category: {
        connectOrCreate: {
          where: {
            id: item.category.id,
          },
          create: {
            id: item.category.id,
            name: item.category.name,
            icon: item.category.emoji,
          },
        },
      },
    };
  }

  toUpdateInputWithCategory(
    item: UpdateShoppingListItemWithCategory,
  ): Prisma.ShoppingListItemUpdateInput {
    return {
      state: item.state,
      amount: item.amount,
      unit: item.unit,
      category: item.category
        ? {
            connectOrCreate: {
              where: {
                id: item.category.id,
              },
              create: {
                id: item.category.id,
                name: item.category.name,
                icon: item.category.emoji,
              },
            },
          }
        : undefined,
    };
  }

  toUpdateInput(
    item: UpdateShoppingListItem,
  ): Prisma.ShoppingListItemUpdateInput {
    return {
      state: item.state,
      amount: item.amount,
      unit: item.unit,
    };
  }
}
