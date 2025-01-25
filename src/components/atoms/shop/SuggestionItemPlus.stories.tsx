import type { Meta, StoryObj } from "@storybook/react";
import SuggestionItemPlus, { State } from "./SuggestionItemPlus";

const meta = {
  title: "Atoms/Shop/SuggestionItemPlus",
  component: SuggestionItemPlus,
  parameters: {
    layout: "centered",
  },
  args: {},
} satisfies Meta<typeof SuggestionItemPlus>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WhenItemIsNotAdded: Story = {
  args: {
    state: State.Default,
  },
};
export const WhenItemIsAlreadyInTheList: Story = {
  args: {
    state: State.Added,
  },
};
