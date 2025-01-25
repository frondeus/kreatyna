import { ShoppingListItem } from "@/lib/types";
import { Group, Paper, Text } from "@mantine/core";

export default function Item({ item }: { item: ShoppingListItem }) {
  const amount = amountAndUnit(item);
  return (
    <Paper>
      <Group grow>
        <Text>{item.name}</Text>
        <Text>{amount}</Text>
      </Group>
    </Paper>
  );
}

function amountAndUnit({ amount, unit }: ShoppingListItem): string {
  if (unit === undefined && amount <= 0) {
    return "";
  }
  if (unit === undefined) {
    return `${amount}`;
  }
  return `${amount}${unit}`;
}
