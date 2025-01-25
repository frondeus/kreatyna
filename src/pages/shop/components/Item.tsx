import { Paper } from "@mantine/core";

export function Item({ item }: { item: any }) {
    return <Paper>
        {item.title}
        {JSON.stringify(item)}
    </Paper>;
}