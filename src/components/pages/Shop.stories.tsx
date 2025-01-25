import { AddShoppingListItem, ShoppingListItemState } from "@/lib/types";
import { APIContext } from "@lib/api";
import type { Meta, StoryObj } from "@storybook/react";
import ShopPage from "./Shop";
const suggestions: AddShoppingListItem[] = [
  {
    kind: "item",
    id: "1",
    name: "Raisin, seedless",
    amount: 50,
    state: ShoppingListItemState.Bought,
  },
  {
    kind: "item",
    id: "2",
    name: "Chestnut, raw",
    amount: 50,
    unit: "g",
    state: ShoppingListItemState.InCart,
  },
  {
    kind: "item",
    id: "3",
    name: "Czekolada gorzka",
    amount: 100,
    unit: "g",
    state: ShoppingListItemState.NotBuying,
  },
  {
    kind: "suggestion",
    id: "4",
    name: "Rye bread, dark, wholemeal, imported",
    amount: 373.2,
    unit: "g",
  },
  {
    kind: "item",
    id: "5",
    name: "Honey",
    amount: 0,
    unit: "g",
    state: ShoppingListItemState.Todo,
  },
];
const meta = {
  title: "Pages/Shop",
  component: ShopPage,
  parameters: {
    // More on how to position stories at: https://storybook.js.org/docs/configure/story-layout
    layout: "fullscreen",
  },
} satisfies Meta<typeof ShopPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NoLists: Story = {};
export const WithLists: Story = {
  decorators: [
    (Story) => (
      <APIContext.Provider
        value={{
          getShoppingLists: async () => [
            {
              id: 1,
              title: "Test List",
              kind: "shoppingList",
              items: [],
            },
          ],
          getItemsAndSuggestions: async () => suggestions,
          subscribeToEvents: () => () => {},
          updateDocument: async (id, changes) => {
            console.log("Update document", id, changes);
          },
        }}
      >
        <Story />
      </APIContext.Provider>
    ),
  ],
};
