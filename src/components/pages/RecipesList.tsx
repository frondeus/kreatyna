import { useAPI } from "@/lib/api";
import { useRoute } from "@/lib/state";
import { Recipe } from "@/lib/types";
import { ActionIcon, Affix, Menu } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";
import { useSuspenseQuery } from "@tanstack/react-query";
import {
  MantineReactTable,
  MRT_ColumnDef,
  useMantineReactTable,
} from "mantine-react-table";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

export default function RecipesList() {
  const goto = useRoute((s) => s.goto);
  const api = useAPI();
  const { data: recipes } = useSuspenseQuery({
    queryKey: ["recipes"],
    queryFn: () => api.getDocuments("recipe"),
  });

  const { t } = useTranslation();

  const data = useMemo(() => recipes.map((r) => r as Recipe), [recipes]);

  const columns = useMemo<MRT_ColumnDef<Recipe>[]>(
    () => [
      {
        id: "name",
        header: t("pages.recipes-list.name"),
        accessorKey: "name",
      },
    ],
    [],
  );

  const table = useMantineReactTable({
    columns,
    data,
    enableColumnOrdering: true,
    enableGlobalFilter: true,
    positionPagination: "top",
    initialState: {
      density: "md",
      pagination: {
        pageIndex: 0,
        pageSize: 25,
      },
    },
    enableRowActions: true,
    positionActionsColumn: "last",
    renderRowActionMenuItems: ({ row }) => (
      <>
        <Menu.Item onClick={goto({ type: "recipe", id: row.original.id })}>
          {t("pages.recipes-list.edit")}
        </Menu.Item>
      </>
    ),
  });

  const newRecipe = async () => {
    const id = await api.newDocument("recipe");
    goto({ type: "recipe", id })();
  };

  return (
    <>
      <MantineReactTable table={table} />
      <Affix position={{ bottom: 78, right: 20 }}>
        <ActionIcon onClick={newRecipe} size="xl" radius="xl">
          <IconPlus />
        </ActionIcon>
      </Affix>
    </>
  );
}
