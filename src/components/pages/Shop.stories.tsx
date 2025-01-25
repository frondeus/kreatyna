import { APIContext } from "@lib/api";
import type { Meta, StoryObj } from "@storybook/react";
import ShopPage from "./Shop";

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
    (story) => (
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
          createShoppingList: async () => 2,
          getItemSuggestions: async () => [],
        }}
      >
        {story()}
      </APIContext.Provider>
    ),
  ],
};
