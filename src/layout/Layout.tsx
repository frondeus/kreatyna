import { ActionIcon, AppShell, Center, CloseButton, CloseIcon, Group, MantineProvider, Title, Text } from "@mantine/core";
import { useDisclosure } from '@mantine/hooks';
import { IconLayoutSidebarLeftExpand, IconLayoutSidebarRightCollapse, IconShoppingCart, IconGrill } from '@tabler/icons-react';
import { useRoute } from '../state';
import { Header } from './Header';

export function Layout({ children }: { children: React.ReactNode }) {
    const [navOpened, { toggle: toggleNav }] = useDisclosure();
    const { setRoute } = useRoute();
    return <AppShell padding="md"
        header={{
            height: 60,
            offset: true
        }}
        navbar={{
            width: 300,
            breakpoint: "sm",
            collapsed: { desktop: !navOpened, mobile: !navOpened },
        }}>
        <AppShell.Navbar p="md">
            <Group justify="flex-end">
                <CloseButton onClick={toggleNav} variant="transparent" hiddenFrom="sm" />
            </Group>
            Navbar
        </AppShell.Navbar>
        <AppShell.Header p="md">
            <Header toggleNav={toggleNav} navOpened={navOpened} />
        </AppShell.Header>
        <AppShell.Main>
            {children}
        </AppShell.Main>
        <AppShell.Footer p="md" hiddenFrom="sm">
            <Group justify="space-between" align="center">
                <ActionIcon variant="subtle" onClick={toggleNav}>
                    {!navOpened ? <IconLayoutSidebarLeftExpand size={16} /> : <IconLayoutSidebarRightCollapse size={16} />}
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

}