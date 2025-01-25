import { useRoute } from "@/lib/state";
import { ActionIcon, Group } from "@mantine/core";
import {
  IconLayoutSidebarLeftExpand,
  IconLayoutSidebarRightCollapse,
} from "@tabler/icons-react";
import { useTranslation } from "react-i18next";

export default function Header({
  toggleNav,
  navOpened,
}: {
  toggleNav: () => void;
  navOpened: boolean;
}) {
  const { route } = useRoute();
  const { t } = useTranslation();

  const routeName = t(`routes.${route}`);

  return (
    <Group justify="center" align="center" grow>
      <Group visibleFrom="sm">
        <ActionIcon variant="subtle" onClick={toggleNav}>
          {!navOpened ? (
            <IconLayoutSidebarLeftExpand size={16} />
          ) : (
            <IconLayoutSidebarRightCollapse size={16} />
          )}
        </ActionIcon>
      </Group>
      <Group style={{ whiteSpace: "nowrap" }}>{routeName}</Group>
      <Group visibleFrom="sm">
        {/* <ActionIcon variant="subtle" visibleFrom="sm"> </ActionIcon> */}
      </Group>
    </Group>
  );
}
