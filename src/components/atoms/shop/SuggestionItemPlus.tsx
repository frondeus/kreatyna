import { ActionIcon } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";

export enum State {
    Default = 'default',
    Added = 'added'
};

export interface SuggestionItemPlusProps {
    state: State;
    onClick?: () => void;
}

export default function SuggestionItemPlus({state = State.Default, onClick}: SuggestionItemPlusProps) {
    let Icon = IconPlus;
    let variant = "subtle";
    if (state === State.Added) {
        Icon = IconPlus;
        variant = "filled";
    }
    return (
        <ActionIcon variant={variant} radius="xl" onClick={onClick}>
            <Icon size="16"/>
        </ActionIcon>
    )
}