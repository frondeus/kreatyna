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
    suggestion: {
      id: "1",
      name: "Raisin, seedless",
      amount: 50,
      unit: "g",
    },
    onAdd: () => {},
  },
} satisfies Meta<typeof SuggestionItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const NoAmount: Story = {
  args: {
    suggestion: {
      ...meta.args.suggestion,
      amount: 0,
    },
  },
};

export const NoAmountAndUnit: Story = {
  args: {
    suggestion: {
      ...meta.args.suggestion,
      amount: 0,
      unit: undefined,
    },
  },
};

export const AddedWithoutAmountOrUnit: Story = {
  args: {
    suggestion: {
      ...meta.args.suggestion,
      amount: 1,
      unit: undefined,
    },
  },
};
