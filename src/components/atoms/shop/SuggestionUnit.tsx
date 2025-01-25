import { ActionIcon, Text, TextInput } from "@mantine/core";
import { IconScale } from "@tabler/icons-react";

export interface SuggestionUnitProps {
  amount: number;
  unit?: string;
  editable?: boolean;
  onClick?: () => void;
}

function amountAndUnit(amount: number, unit?: string) {
  const unitString = unit ?? "";
  return `${amount}${unitString}`;
}

export default function SuggestionUnit({
  amount,
  unit,
  editable = false,
  onClick,
}: SuggestionUnitProps) {
  const showAmount = amount > 1 || unit !== undefined;

  if (editable) {
    if (showAmount) {
      return (
        <TextInput
          w="5em"
          onClick={onClick}
          value={amountAndUnit(amount, unit)}
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
