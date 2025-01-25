import Header from "@components/_layout/Header";
import { useRoute } from "@lib/state";
import { ActionIcon, AppShell, CloseButton, Group } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import {
  IconGrill,
  IconLayoutSidebarLeftExpand,
  IconLayoutSidebarRightCollapse,
  IconShoppingCart,
} from "@tabler/icons-react";

export default function Layout({ children }: { children: React.ReactNode }) {
  const [navOpened, { toggle: toggleNav }] = useDisclosure();
  const { setRoute } = useRoute();
  return (
    <AppShell
      padding="md"
      header={{
        height: 60,
        offset: true,
      }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { desktop: !navOpened, mobile: !navOpened },
      }}
    >
      <AppShell.Navbar p="md">
        <Group justify="flex-end">
          <CloseButton
            onClick={toggleNav}
            variant="transparent"
            hiddenFrom="sm"
          />
        </Group>
        Navbar
      </AppShell.Navbar>
      <AppShell.Header p="md">
        <Header toggleNav={toggleNav} navOpened={navOpened} />
      </AppShell.Header>
      <AppShell.Main>{children}</AppShell.Main>
      <AppShell.Footer p="md" hiddenFrom="sm">
        <Group justify="space-between" align="center">
          <ActionIcon variant="subtle" onClick={toggleNav}>
            {!navOpened ? (
              <IconLayoutSidebarLeftExpand size={16} />
            ) : (
              <IconLayoutSidebarRightCollapse size={16} />
            )}
          </ActionIcon>

          <ActionIcon variant="subtle" onClick={() => setRoute("shop")}>
            <IconShoppingCart size={16} />
          </ActionIcon>

          <ActionIcon variant="subtle">
            <IconGrill size={16} />
          </ActionIcon>
        </Group>
      </AppShell.Footer>
    </AppShell>
  );
}
