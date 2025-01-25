import { ShoppingListItemState } from "@/lib/types";
import { ActionIcon } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";

export enum State {
  Default = "default",
  Added = "added",
}

export interface SuggestionItemPlusProps {
  state: State;
  itemState?: ShoppingListItemState;
  onClick?: () => void;
}

const STATES = {
  [ShoppingListItemState.Bought]: "gray",
  [ShoppingListItemState.NotBuying]: "red",
  [ShoppingListItemState.Todo]: "blue",
  [ShoppingListItemState.InCart]: "green",
};

export default function SuggestionItemPlus({
  state = State.Default,
  itemState,
  onClick,
}: SuggestionItemPlusProps) {
  let Icon = IconPlus;
  let variant = "subtle";
  let color = "blue";
  if (state === State.Added) {
    Icon = IconPlus;
    variant = "filled";
    if (itemState) color = STATES[itemState];
  }
  return (
    <ActionIcon variant={variant} radius="xl" onClick={onClick} color={color}>
      <Icon size="16" />
    </ActionIcon>
  );
}
