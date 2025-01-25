import type { Meta, StoryObj } from "@storybook/react";
import SuggestionUnit from "./SuggestionUnit";

const meta = {
  title: "Atoms/Shop/SuggestionUnit",
  component: SuggestionUnit,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  args: {},
} satisfies Meta<typeof SuggestionUnit>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WhenAmountIs1AndNoUnit: Story = {
  args: {
    amount: 1,
  },
};

export const WhenAmountIsGreaterThan1AndNoUnit: Story = {
  args: {
    amount: 2,
  },
};

export const WhenAmountIs1AndHasUnit: Story = {
  args: {
    amount: 1,
    unit: "kg",
  },
};

export const WhenAmountIsGreaterThan1AndHasUnit: Story = {
  args: {
    amount: 2,
    unit: "kg",
  },
};

export const WhenEditableAndAmountIs1AndNoUnit: Story = {
  args: {
    amount: 1,
    editable: true,
  },
};

export const WhenEditableAndAmountIsGreaterThan1AndNoUnit: Story = {
  args: {
    amount: 2,
    editable: true,
  },
};

export const WhenEditableAndAmountIs1AndHasUnit: Story = {
  args: {
    amount: 1,
    unit: "kg",
    editable: true,
  },
};

export const WhenEditableAndAmountIsGreaterThan1AndHasUnit: Story = {
  args: {
    amount: 2,
    unit: "kg",
    editable: true,
  },
};
