import { Paper } from "@mantine/core";
import { ShoppingListItem } from "../../../types";

export function Item({ item }: { item: ShoppingListItem }) {
    return <Paper>
        {item}
    </Paper>;
}