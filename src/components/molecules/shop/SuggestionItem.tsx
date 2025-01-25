import SuggestionItemPlus, {
  State as CheckboxState,
} from "@components/atoms/shop/SuggestionItemPlus";
import SuggestionUnit from "@components/atoms/shop/SuggestionUnit";
import { ShoppingListSuggestion } from "@lib/types";
import { Group, Paper, Text } from "@mantine/core";

export interface SuggestionItemProps {
  suggestion: ShoppingListSuggestion;
  onAdd: () => void;
}

export default function SuggestionItem({
  suggestion,
  onAdd,
}: SuggestionItemProps) {
  const onPlusClick = () => {
    onAdd();
  };

  return (
    <Paper mb="xs">
      <Group justify="space-between" wrap="nowrap">
        <SuggestionItemPlus
          state={CheckboxState.Default}
          onClick={onPlusClick}
        />
        <Text size="sm" style={{ flex: 1 }}>
          {suggestion.name}
        </Text>
        <Group>
          <SuggestionUnit amount={suggestion.amount} unit={suggestion.unit} />
        </Group>
      </Group>
    </Paper>
  );
}
