import { ShoppingListItemState } from "@lib/types";
import type { Meta, StoryObj } from "@storybook/react";
import SuggestionItem from "./SuggestionItem";

const meta = {
  title: "Molecules/Shop/SuggestionItem",
  component: SuggestionItem,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  args: {
    name: "Raisin, seedless",
    amount: 50,
    unit: "g",
    state: ShoppingListItemState.Todo,
    onIncrease: () => {},
    onRemove: () => {},
    onAdd: () => {},
    onDecrease: () => {},
    onEditUnit: () => {},
  },
} satisfies Meta<typeof SuggestionItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const NoAmount: Story = {
  args: {
    amount: undefined,
  },
};

export const NoAmountAndUnit: Story = {
  args: {
    ...NoAmount.args,
    unit: undefined,
  },
};

export const AddedWithoutAmountOrUnit: Story = {
  args: {
    state: ShoppingListItemState.InCart,
    amount: 1,
    unit: undefined,
  },
};
