import { Route, useRoute } from "@/lib/state";
import {
  ActionIcon,
  Anchor,
  Breadcrumbs,
  Group,
  Text,
  useMatches,
} from "@mantine/core";
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
  const { route, setRoute } = useRoute();
  const { t } = useTranslation();

  const crumbs = breadcrumbs(route)
    .map((crumb) => ({
      title: t(`routes.${crumb.title}`),
      href: crumb.href,
    }))
    .map((crumb, index) => {
      if (!crumb.href) {
        return <Text key={index}>{crumb.title}</Text>;
      }
      const href = crumb.href;

      return (
        <Anchor key={index} onClick={() => setRoute(href)}>
          {crumb.title}
        </Anchor>
      );
    });

  const show =
    useMatches({
      xs: false,
      sm: true,
    }) ?? false;

  return (
    <Group justify="center" align="center" grow>
      {show && (
        <Group>
          <ActionIcon variant="subtle" onClick={toggleNav}>
            {!navOpened ? (
              <IconLayoutSidebarLeftExpand size={16} />
            ) : (
              <IconLayoutSidebarRightCollapse size={16} />
            )}
          </ActionIcon>
        </Group>
      )}
      <Group style={{ whiteSpace: "nowrap" }} align="center" justify="center">
        <Breadcrumbs>{crumbs}</Breadcrumbs>
      </Group>
      {show && (
        <Group>
          {/* <ActionIcon variant="subtle" visibleFrom="sm"> </ActionIcon> */}
        </Group>
      )}
    </Group>
  );
}

interface Breadcrumb {
  title: string;
  href?: Route;
}

function breadcrumbs(route: Route): Breadcrumb[] {
  if (typeof route === "string") {
    return [{ title: route }];
  }
  switch (route.type) {
    case "raw-ingredient": {
      return [
        { title: "raw-ingredient-list", href: "raw-ingredient-list" },
        { title: "raw-ingredient" },
      ];
    }
  }
}
