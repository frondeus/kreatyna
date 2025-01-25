import { Modal, Stack, TextInput } from "@mantine/core";

export interface AddItemModalProps {
    opened: boolean;
    onClose: () => void;
}

export function AddItemModal({ opened, onClose }: AddItemModalProps) {
    return (<Modal 
        opened={opened} 
        onClose={onClose}
        fullScreen>
        <Stack>
            <TextInput />
        </Stack>
    </Modal>);
}