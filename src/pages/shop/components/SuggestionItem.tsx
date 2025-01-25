import { ActionIcon, Group, Paper, Text } from "@mantine/core";
import { IconMinus, IconPlus, IconScale } from "@tabler/icons-react";
import { ShoppingListItem } from "../../../types";

export interface SuggestionItemProps extends Omit<ShoppingListItem, 'id'> {
    onAdd?: () => void;
    onRemove?: () => void;
}

export function SuggestionItem({ name, amount, onAdd, onRemove, hasScale = false }: SuggestionItemProps) {
    return (
        <Paper p="xs" withBorder>
            <Group justify="space-between" wrap="nowrap">
                <Text size="sm" style={{ flex: 1 }}>{name}</Text>
                <Group gap="xs" wrap="nowrap">
                    {amount && (
                        <Text size="sm" c="dimmed">{amount}</Text>
                    )}
                    {hasScale && (
                        <ActionIcon variant="subtle" color="gray" size="md">
                            <IconScale size={16} />
                        </ActionIcon>
                    )}
                    {onRemove && (
                        <ActionIcon variant="subtle" color="gray" size="md" onClick={onRemove}>
                            <IconMinus size={16} />
                        </ActionIcon>
                    )}
                    {onAdd && (
                        <ActionIcon variant="subtle" color="gray" size="md" onClick={onAdd}>
                            <IconPlus size={16} />
                        </ActionIcon>
                    )}
                </Group>
            </Group>
        </Paper>
    );
} 