import { ActionIcon, Group, Paper, Text } from "@mantine/core";
import { IconMinus,  IconX } from "@tabler/icons-react";
import { ShoppingListItem, ShoppingListItemState } from "../../../lib/types";
import SuggestionItemPlus, { State as CheckboxState } from "../../atoms/shop/SuggestionItemPlus";
import SuggestionUnit from "../../atoms/shop/SuggestionUnit";

export interface SuggestionItemProps extends Omit<ShoppingListItem, 'id'> {
    onIncrease: () => void;
    onDecrease: () => void;
    onRemove: () => void;
    onAdd: () => void;
    onEditUnit: () => void;
}

export default function SuggestionItem({ name, amount = 0, unit, state, onAdd, onIncrease, onRemove, onDecrease, onEditUnit }: SuggestionItemProps) {
    let checkboxState: CheckboxState = CheckboxState.Default;
    if (amount > 0) {
        checkboxState = CheckboxState.Added;
    }

    let reducingControls: React.ReactNode = null;
    if (state !== ShoppingListItemState.Bought && state !== ShoppingListItemState.Todo) {
        let clickHandler;
        let Icon;
        if (amount > 0 ) {
            Icon = IconMinus;
            clickHandler = onDecrease;
        } else {
            Icon = IconX;
            clickHandler = onRemove;
        }
        reducingControls = <ActionIcon variant="light" color="red" onClick={clickHandler} radius="xl">
            <Icon/>
        </ActionIcon>;
    }

    const onPlusClick = () => {
        if (amount > 0) {
            onIncrease();
        }
        else {
            onAdd();
        }
    }

    return (
        <Paper p="xs" withBorder>
            <Group justify="space-between" wrap="nowrap">
                <SuggestionItemPlus state={checkboxState} onClick={onPlusClick} />
                <Text size="sm" style={{ flex: 1 }}> {name} </Text>
                <Group>
                    <SuggestionUnit amount={amount} unit={unit} onClick={onEditUnit} editable={
                        amount > 0 && state !== ShoppingListItemState.Bought
                    } />
                    {reducingControls}
                </Group>
            </Group>
        </Paper>
    );
} 