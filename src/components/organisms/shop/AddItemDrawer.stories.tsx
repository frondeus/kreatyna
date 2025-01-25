import { ShoppingListItemState } from "@/lib/types";
import { APIContext } from "@lib/api";
import type { Meta, StoryObj } from "@storybook/react";
import AddItemDrawer from "./AddItemDrawer";

const suggestions = [
  {
    id: "1",
    name: "Raisin, seedless",
    amount: 50,
    state: ShoppingListItemState.Bought,
  },
  {
    id: "2",
    name: "Chestnut, raw",
    amount: 50,
    unit: "g",
    state: ShoppingListItemState.InCart,
  },
  {
    id: "3",
    name: "Czekolada gorzka",
    amount: 100,
    unit: "g",
    state: ShoppingListItemState.NotBuying,
  },
  {
    id: "4",
    name: "Rye bread, dark, wholemeal, imported",
    amount: 373.2,
    unit: "g",
  },
  {
    id: "5",
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
          createShoppingList: async () => 1,
          getItemSuggestions: async (_query) => {
            await new Promise((resolve) => setTimeout(resolve, 500));
            return suggestions;
          },
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
