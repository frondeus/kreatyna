import { ActionIcon, Affix, ScrollArea, Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconPlus } from "@tabler/icons-react";
import "./ListPage.css";

export interface RenderModalProps {
  opened: boolean;
  onClose: () => void;
}

export interface ListPageProps {
  children: React.ReactNode;
  renderModal(renderModalProps: RenderModalProps): React.ReactNode;
}
export default function ListPage({ children, renderModal }: ListPageProps) {
  const [openedAddingModal, { toggle: toggleAddingModal }] =
    useDisclosure(false);

  return (
    <ScrollArea
      h="100%"
      styles={{
        root: {
          flex: "1",
          display: "flex",
          flexDirection: "column",
        },
        viewport: {
          flex: "1",
          display: "flex",
          flexDirection: "column",
        },
      }}
    >
      <Stack bg="dark.8" h="100%" gap="0" flex="1">
        {children}
        <Affix position={{ bottom: 78, right: 20 }}>
          <ActionIcon onClick={toggleAddingModal} size="xl" radius="xl">
            <IconPlus />
          </ActionIcon>
        </Affix>
        {renderModal({ opened: openedAddingModal, onClose: toggleAddingModal })}
      </Stack>
    </ScrollArea>
  );
}
