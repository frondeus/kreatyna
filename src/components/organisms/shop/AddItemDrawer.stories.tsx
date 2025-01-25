import { APIContext } from "@/lib/api";
import { AddShoppingListItem, ShoppingListItemState } from "@/lib/types";
import type { Meta, StoryObj } from "@storybook/react";
import AddItemDrawer from "./AddItemDrawer";

const suggestions: AddShoppingListItem[] = [
  {
    id: "1",
    kind: "item",
    name: "Raisin, seedless",
    amount: 50,
    state: ShoppingListItemState.Bought,
  },
  {
    id: "2",
    kind: "item",
    name: "Chestnut, raw",
    amount: 50,
    unit: "g",
    state: ShoppingListItemState.InCart,
  },
  {
    id: "3",
    kind: "item",
    name: "Czekolada gorzka",
    amount: 100,
    unit: "g",
    state: ShoppingListItemState.NotBuying,
  },
  {
    id: "4",
    kind: "suggestion",
    name: "Rye bread, dark, wholemeal, imported",
    amount: 373.2,
    unit: "g",
  },
  {
    id: "5",
    kind: "item",
    name: "Honey",
    amount: 0,
    unit: "g",
    state: ShoppingListItemState.Todo,
  },
];

const meta = {
  title: "Organisms/Shop/AddItemDrawer",
  component: AddItemDrawer,
  parameters: {
    layout: "fullscreen",
  },
  args: {
    onClose: () => {},
    opened: true,
  },
  decorators: [
    (Story) => (
      <APIContext.Provider
        value={{
          getShoppingLists: async () => [],
          getItemsAndSuggestions: async (_query) => {
            await new Promise((resolve) => setTimeout(resolve, 500));
            return suggestions;
          },
          subscribeToEvents: () => () => {},
          updateDocument: async () => {},
        }}
      >
        <Story />
      </APIContext.Provider>
    ),
  ],
} satisfies Meta<typeof AddItemDrawer>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
