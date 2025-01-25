import '@mantine/core/styles.css';

import "./App.css";
import { ActionIcon, AppShell, Center, CloseButton, CloseIcon, Group, MantineProvider, Title, Text } from "@mantine/core";
import { useDisclosure } from '@mantine/hooks';
import { IconLayoutSidebarLeftExpand, IconLayoutSidebarRightCollapse, IconShoppingCart, IconGrill } from '@tabler/icons-react';

function App() {
  const [navOpened, { toggle: toggleNav }] = useDisclosure();

  return (
    <MantineProvider defaultColorScheme="dark">
      <AppShell padding="md" navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { desktop: !navOpened, mobile: !navOpened },
      }}>
        <AppShell.Navbar p="md">
          <Group justify="flex-end">
            <CloseButton onClick={toggleNav} variant="transparent" hiddenFrom="sm"  />
          </Group>
          Navbar
        </AppShell.Navbar>
        <AppShell.Header p ="md">
          <Group justify="space-between" align="center">
            <ActionIcon variant="subtle" onClick={toggleNav} visibleFrom="sm">
              {!navOpened ? <IconLayoutSidebarLeftExpand size={16} /> : <IconLayoutSidebarRightCollapse size={16} />}
            </ActionIcon>
            <Text>Header</Text>
            <Text visibleFrom="sm"></Text>
            {/* <ActionIcon variant="subtle" visibleFrom="sm"> </ActionIcon> */}
          </Group>
          
        </AppShell.Header>
        <AppShell.Main>
        </AppShell.Main>
        <AppShell.Footer p="md" hiddenFrom="sm">
            <Group justify="space-between" align="center">
              <ActionIcon variant="subtle" onClick={toggleNav}>
                {!navOpened ? <IconLayoutSidebarLeftExpand size={16} /> : <IconLayoutSidebarRightCollapse size={16} />}
              </ActionIcon>

              <ActionIcon variant="subtle">
                <IconShoppingCart size={16} />
              </ActionIcon>

              <ActionIcon variant="subtle">
                <IconGrill size={16} />
              </ActionIcon>
            </Group>
          </AppShell.Footer>
      </AppShell>
    </MantineProvider>
  )
}

export default App;
