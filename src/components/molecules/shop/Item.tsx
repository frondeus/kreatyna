import { ShoppingListItem } from "@lib/types";
import { Paper } from "@mantine/core";

export default function Item({ item }: { item: ShoppingListItem }) {
  return <Paper>{item.name}</Paper>;
}
