import { Paper } from "@mantine/core";
import { ShoppingListItem } from "../../../lib/types";

export default function Item({ item }: { item: ShoppingListItem }) {
    return <Paper>
        {item.name}
    </Paper>;
}