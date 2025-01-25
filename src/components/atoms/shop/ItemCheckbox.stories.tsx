import { ShoppingListItemState } from "@/lib/types";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import ItemCheckbox from "./ItemCheckbox";

function ItemCheckboxWrapper({
  initialState,
}: {
  initialState: ShoppingListItemState;
}) {
  const [state, setState] = useState(initialState);
  return <ItemCheckbox state={state} onChange={setState} />;
}

const meta = {
  title: "Atoms/Shop/ItemCheckbox",
  component: ItemCheckboxWrapper,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  args: {},
} satisfies Meta<typeof ItemCheckboxWrapper>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Todo: Story = {
  args: {
    initialState: ShoppingListItemState.Todo,
  },
};

export const Bought: Story = {
  args: {
    initialState: ShoppingListItemState.Bought,
  },
};

export const InCart: Story = {
  args: {
    initialState: ShoppingListItemState.InCart,
  },
};

export const NotBuying: Story = {
  args: {
    initialState: ShoppingListItemState.NotBuying,
  },
};
