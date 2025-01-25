import { APIContext } from "@/lib/api";
import { useRawIngredients, useRoute } from "@/lib/state";
import { RawIngredient } from "@/lib/types";
import { Menu } from "@mantine/core";
import {
  MantineReactTable,
  MRT_ColumnDef,
  useMantineReactTable,
} from "mantine-react-table";
import { useContext, useMemo } from "react";
import { useTranslation } from "react-i18next";
import "./RawIngredientList.css";

export default function RawIngredientList() {
  const goto = useRoute((s) => s.goto);
  const api = useContext(APIContext);
  const fetch = useRawIngredients((s) => s.fetch);
  const rawIngredients = fetch(api.getRawIngredients);
  const { t } = useTranslation();

  const data = useMemo(() => rawIngredients, [rawIngredients]);
  const columns = useMemo<MRT_ColumnDef<RawIngredient>[]>(
    () => [
      {
        id: "name",
        header: t("pages.raw-ingredient-list.name"),
        accessorKey: "name",
      },
      {
        id: "proteins",
        header: t("pages.raw-ingredient-list.proteins"),
        accessorFn: (row) => row.proteins.toFixed(2),
      },
      {
        id: "carbs",
        header: t("pages.raw-ingredient-list.carbs"),
        accessorFn: (row) => row.carbs.toFixed(2),
      },
      {
        id: "fat",
        header: t("pages.raw-ingredient-list.fat"),
        accessorFn: (row) => row.fat.toFixed(2),
      },
    ],
    [],
  );

  const table = useMantineReactTable({
    columns,
    data,
    enableColumnOrdering: true,
    enableGlobalFilter: true,
    initialState: {
      density: "md",
      pagination: {
        pageIndex: 0,
        pageSize: 25,
      },
      columnVisibility: {
        proteins: false,
        carbs: false,
        fat: false,
      },
    },
    positionPagination: "top",
    enableRowActions: true,
    renderRowActionMenuItems: ({ row }) => (
      <>
        <Menu.Item
          onClick={goto({
            type: "raw-ingredient",
            id: row.original.id.toString(),
          })}
        >
          {t("pages.raw-ingredient-list.edit")}
        </Menu.Item>
      </>
    ),
    positionActionsColumn: "last",
  });

  return <MantineReactTable table={table} />;
}
