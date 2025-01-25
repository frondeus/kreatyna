import { useAPI } from "@/lib/api";
import { Recipe } from "@/lib/types";
import { useSuspenseQuery } from "@tanstack/react-query";
import {
  MantineReactTable,
  MRT_ColumnDef,
  useMantineReactTable,
} from "mantine-react-table";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

export default function RecipesList() {
  //   const goto = useRoute((s) => s.goto);
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
  });

  return <MantineReactTable table={table} />;
}
