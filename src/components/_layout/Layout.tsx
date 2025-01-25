import Header from "@/components/_layout/Header";
import { useRoute } from "@/lib/state";
import { ActionIcon, AppShell, CloseButton, Group } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import {
  IconGrill,
  IconLayoutSidebarLeftExpand,
  IconLayoutSidebarRightCollapse,
  IconShoppingCart,
} from "@tabler/icons-react";
import Navbar from "./Navbar";

export default function Layout({ children }: { children: React.ReactNode }) {
  const [navOpened, { toggle: toggleNav }] = useDisclosure();
  const goto = useRoute((s) => s.goto);
  console.log("Layout");
  return (
    <AppShell
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
        <Navbar onClose={toggleNav} />
      </AppShell.Navbar>
      <AppShell.Header p="md">
        <Header toggleNav={toggleNav} navOpened={navOpened} />
      </AppShell.Header>
      <AppShell.Main
        style={{ display: "flex", flexDirection: "column", height: "100%" }}
      >
        {children}
      </AppShell.Main>
      <AppShell.Footer p="md" hiddenFrom="sm">
        <Group justify="space-between" align="center">
          <ActionIcon variant="subtle" onClick={toggleNav}>
            {!navOpened ? (
              <IconLayoutSidebarLeftExpand size={16} />
            ) : (
              <IconLayoutSidebarRightCollapse size={16} />
            )}
          </ActionIcon>

          <ActionIcon variant="subtle" onClick={goto("shop")}>
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
