import SuggestionItemPlus, {
  State as CheckboxState,
} from "@components/atoms/shop/SuggestionItemPlus";
import SuggestionUnit from "@components/atoms/shop/SuggestionUnit";
import { ShoppingListItemState, ShoppingListSuggestion } from "@lib/types";
import { ActionIcon, Group, Paper, Text } from "@mantine/core";
import { IconMinus, IconX } from "@tabler/icons-react";

export interface SuggestionItemProps extends ShoppingListSuggestion {
  onIncrease: () => void;
  onDecrease: () => void;
  onRemove: () => void;
  onAdd: () => void;
  onEditUnit: () => void;
}

export default function SuggestionItem({
  name,
  amount = 0,
  unit,
  state,
  onAdd,
  onIncrease,
  onRemove,
  onDecrease,
  onEditUnit,
}: SuggestionItemProps) {
  let checkboxState: CheckboxState = CheckboxState.Default;
  const isAdded = state !== undefined && amount > 0;
  if (isAdded) {
    checkboxState = CheckboxState.Added;
  }

  let reducingControls: React.ReactNode = null;
  if (isAdded && state !== ShoppingListItemState.Bought) {
    let clickHandler;
    let Icon;
    if (amount > 0) {
      Icon = IconMinus;
      clickHandler = onDecrease;
    } else {
      Icon = IconX;
      clickHandler = onRemove;
    }
    reducingControls = (
      <ActionIcon
        variant="light"
        color="red"
        onClick={clickHandler}
        radius="xl"
      >
        <Icon />
      </ActionIcon>
    );
  }

  const onPlusClick = () => {
    if (amount > 0) {
      onIncrease();
    } else {
      onAdd();
    }
  };

  return (
    <Paper mb="xs">
      <Group justify="space-between" wrap="nowrap">
        <SuggestionItemPlus state={checkboxState} onClick={onPlusClick} />
        <Text size="sm" style={{ flex: 1 }}>
          {" "}
          {name}{" "}
        </Text>
        <Group>
          <SuggestionUnit
            amount={amount}
            unit={unit}
            onClick={onEditUnit}
            editable={isAdded}
          />
          {reducingControls}
        </Group>
      </Group>
    </Paper>
  );
}
