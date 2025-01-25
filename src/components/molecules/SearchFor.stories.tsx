import { APIContext } from "@/lib/api";
import type { Document } from "@/lib/types";
import type { Meta, StoryObj } from "@storybook/react";
import SearchFor from "./SearchFor";

const meta = {
  title: "Molecules/SearchFor",
  component: SearchFor,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  args: {
    kind: "rawIngredient",
    onSelect: (doc) => console.log("Selected:", doc),
  },
} satisfies Meta<typeof SearchFor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
  decorators: [
    (Story) => (
      <APIContext.Provider
        value={{
          getItemsAndSuggestions: async () => [],
          subscribeToEvents: () => () => {},
          subscribeToDocumentEvents: () => () => {},
          finishShopping: () => new Promise<void>(() => {}),
          updateDocument: async () => {},
          getDocuments: async (_) => [
            {
              id: 1,
              title: "Apple",
              kind: "rawIngredient",
              name: "Apple",
              container: 100,
              carbs: 1,
              proteins: 1,
              fat: 0,
            },
          ],
          getDocument: () => new Promise<Document>(() => {}),
          newDocument: () => new Promise<number>(() => {}),
        }}
      >
        <Story />
      </APIContext.Provider>
    ),
  ],
};
