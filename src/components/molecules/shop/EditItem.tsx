import SuggestionItemPlus, {
  State as CheckboxState,
} from "@/components/atoms/shop/SuggestionItemPlus";
import SuggestionUnit from "@/components/atoms/shop/SuggestionUnit";
import { ShoppingListItem } from "@/lib/types";
import { ActionIcon, Group, Paper, Text } from "@mantine/core";
import { IconMinus, IconX } from "@tabler/icons-react";

export interface EditItemProps {
  item: ShoppingListItem;
  onIncrease: () => void;
  onDecrease: () => void;
  onRemove: () => void;
  onEditUnit: () => void;
}

export default function EditItem({
  item,
  onIncrease,
  onDecrease,
  onRemove,
  onEditUnit,
}: EditItemProps) {
  const onPlusClick = () => {
    onIncrease();
  };
  let clickHandler;
  let Icon;
  if (item.amount > 1) {
    Icon = IconMinus;
    clickHandler = onDecrease;
  } else {
    Icon = IconX;
    clickHandler = onRemove;
  }
  return (
    <Paper>
      <Group justify="space-between" wrap="nowrap">
        <SuggestionItemPlus
          state={CheckboxState.Added}
          itemState={item.state}
          onClick={onPlusClick}
        />
        <Text size="sm" style={{ flex: 1 }}>
          {item.name}
        </Text>
        <Group>
          <SuggestionUnit
            amount={item.amount}
            unit={item.unit}
            onClick={onEditUnit}
          />
          <ActionIcon
            variant="light"
            color="red"
            onClick={clickHandler}
            radius="xl"
          >
            <Icon />
          </ActionIcon>
        </Group>
      </Group>
    </Paper>
  );
}
