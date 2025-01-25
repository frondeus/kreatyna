import { ActionIcon, Text, TextInput } from "@mantine/core";
import { IconScale } from "@tabler/icons-react";

export interface SuggestionUnitProps {
  amount: number;
  unit?: string;
  onClick?: () => void;
}

function amountAndUnit(amount: number, unit?: string) {
  const unitString = unit ?? "";
  return `${amount}${unitString}`;
}

export default function SuggestionUnit({
  amount,
  unit,
  onClick,
}: SuggestionUnitProps) {
  const showAmount = amount > 1 || unit !== undefined;
  const editable = onClick !== undefined;

  if (editable) {
    if (showAmount) {
      return (
        <TextInput
          w="5em"
          onClick={onClick}
          value={amountAndUnit(amount, unit)}
          readOnly
        />
      );
    }
    return (
      <ActionIcon onClick={onClick} radius="xl" variant="subtle">
        <IconScale />
      </ActionIcon>
    );
  }

  if (!showAmount) {
    return null;
  }

  return <Text>{amountAndUnit(amount, unit)}</Text>;
}
