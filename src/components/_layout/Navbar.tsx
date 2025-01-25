import { Route, useRoute } from "@/lib/state";
import { NavLink, Stack } from "@mantine/core";
import { IconBook, IconCarrot } from "@tabler/icons-react";
import { useTranslation } from "react-i18next";

export interface NavbarProps {
  onClose: () => void;
}
export default function Navbar({ onClose }: NavbarProps) {
  const { t } = useTranslation();
  const gotoRoute = useRoute((s) => s.goto);
  const goto = (route: Route) => () => {
    gotoRoute(route)();
    onClose();
  };
  return (
    <Stack>
      <NavLink
        onClick={goto("raw-ingredient-list")}
        label={t("routes.raw-ingredient-list")}
        leftSection={<IconCarrot size={16} />}
      />
      <NavLink
        onClick={goto("recipe-list")}
        label={t("routes.recipe-list")}
        leftSection={<IconBook size={16} />}
      />
    </Stack>
  );
}
