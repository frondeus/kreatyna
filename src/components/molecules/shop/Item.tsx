import ItemCheckbox from "@/components/atoms/shop/ItemCheckbox";
import { APIContext } from "@/lib/api";
import { ShoppingListItem, ShoppingListItemState } from "@/lib/types";
import { Group, Paper, Text } from "@mantine/core";
import { useContext } from "react";

const DOCUMENT_ID = 1;

export default function Item({ item }: { item: ShoppingListItem }) {
  const amount = amountAndUnit(item);
  const api = useContext(APIContext);

  const handleStateChange = (state: ShoppingListItemState) => {
    api.updateDocument(DOCUMENT_ID, [
      [
        `items.${item.id}.state`,
        {
          kind: "replace",
          value: state,
        },
      ],
    ]);
  };

  return (
    <Paper>
      <Group grow>
        <Group>
          <ItemCheckbox state={item.state} onChange={handleStateChange} />
          <Text>{item.name}</Text>
        </Group>
        <Group justify="flex-end">
          <Text>{amount}</Text>
        </Group>
      </Group>
    </Paper>
  );
}

function amountAndUnit({ amount, unit }: ShoppingListItem): string {
  if (unit === undefined && amount <= 1) {
    return "";
  }
  if (unit === undefined) {
    return `${amount}`;
  }
  return `${amount}${unit}`;
}
