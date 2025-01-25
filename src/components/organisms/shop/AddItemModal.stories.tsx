import { ShoppingListItemState } from "@/lib/types";
import { APIContext } from "@lib/api";
import type { Meta, StoryObj } from "@storybook/react";
import AddItemModal from "./AddItemModal";

const meta = {
  title: "Organisms/Shop/AddItemModal",
  component: AddItemModal,
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <APIContext.Provider
        value={{
          getShoppingLists: async () => [],
          createShoppingList: async () => 1,
          getItemSuggestions: async (query) => {
            // Simulate API delay
            await new Promise((resolve) => setTimeout(resolve, 500));

            const suggestions = [
              {
                id: "1",
                name: "Raisin, seedless",
                amount: 50,
                unit: "g",
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
                state: ShoppingListItemState.Todo,
              },
              {
                id: "5",
                name: "Honey",
                amount: 20,
                unit: "g",
                state: ShoppingListItemState.Todo,
              },
            ];

            return suggestions.filter((s) =>
              s.name.toLowerCase().includes(query.toLowerCase()),
            );
          },
        }}
      >
        <Story />
      </APIContext.Provider>
    ),
  ],
} satisfies Meta<typeof AddItemModal>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    opened: true,
    onClose: () => {},
  },
};

export const Closed: Story = {
  args: {
    opened: false,
    onClose: () => {},
  },
};
