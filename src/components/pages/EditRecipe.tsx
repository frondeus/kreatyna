import { useAPI } from "@/lib/api";
import { Document, DocumentId, Recipe, RecipeIngredient } from "@/lib/types";
import { onScalarChange, useSubscribeForm } from "@/lib/utils";
import {
  ActionIcon,
  Drawer,
  Fieldset,
  Group,
  Menu,
  NumberInput,
  Paper,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { IconPlus } from "@tabler/icons-react";
import { useSuspenseQuery } from "@tanstack/react-query";
import { MantineReactTable, useMantineReactTable } from "mantine-react-table";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { v4 as uuidv4 } from "uuid";
import SearchFor from "../molecules/SearchFor";

export interface EditRecipeProps {
  id: number;
}

export default function EditRecipe({ id }: EditRecipeProps) {
  const { t } = useTranslation();
  const api = useAPI();
  const { data: recipe } = useSuspenseQuery({
    queryKey: ["recipe", id],
    queryFn: () => api.getDocument(id),
  });

  if (recipe.kind !== "recipe") {
    throw new Error("Recipe not found");
  }

  const form = useForm<Recipe>({
    initialValues: recipe,
  });
  useSubscribeForm(id, api, form);

  const ingredients = form.getInputProps("ingredients");

  return (
    <Paper>
      <Fieldset legend={t("pages.edit-recipe.legend")}>
        <TextInput
          label={t("pages.edit-recipe.name")}
          {...form.getInputProps("name")}
          onChange={onScalarChange(form, api, "name")}
        />
      </Fieldset>
      <Fieldset legend={t("pages.edit-recipe.ingredients")}>
        <EditIngredients
          docId={id}
          ingredients={ingredients.value ?? ingredients.defaultValue}
        />
      </Fieldset>
    </Paper>
  );
}

interface EditIngredientsProps {
  docId: DocumentId;
  ingredients: RecipeIngredient[];
}
function EditIngredients({ docId, ingredients }: EditIngredientsProps) {
  const { t } = useTranslation();
  const [opened, { toggle }] = useDisclosure(false);
  const [editedIngredient, setEditedIngredient] = useState<
    RecipeIngredient | undefined
  >(undefined);
  const data = useMemo(() => ingredients, [ingredients]);
  const api = useAPI();
  const columns = useMemo(
    () => [
      {
        id: "name",
        header: t("pages.edit-recipe.ingredient.name"),
        accessorKey: "name",
      },
    ],
    [],
  );
  const newIngredient = async () => {
    const ingredientId = uuidv4();
    const ingredient: RecipeIngredient = {
      id: ingredientId,
      raw: {
        kind: "rawIngredient",
        id: 0,
        title: "",
        name: "",
        container: 0,
        proteins: 0,
        carbs: 0,
        fat: 0,
      },
    };
    await api.updateDocument(docId, [
      [
        "ingredients",
        {
          kind: "insert",
          key: ingredientId,
          value: ingredient,
        },
      ],
    ]);
    setEditedIngredient(ingredient);
    toggle();
  };
  const table = useMantineReactTable({
    columns,
    data,
    enablePagination: false,
    renderTopToolbarCustomActions: () => (
      <>
        <ActionIcon onClick={newIngredient} variant="subtle">
          <IconPlus />
        </ActionIcon>
      </>
    ),
    enableRowActions: true,
    positionActionsColumn: "last",
    renderRowActionMenuItems: ({ row }) => (
      <>
        <Menu.Item onClick={() => setEditedIngredient(row.original)}>
          {t("pages.edit-recipe.ingredient.edit")}
        </Menu.Item>
      </>
    ),
  });
  return (
    <>
      <MantineReactTable table={table} />
      <Drawer
        opened={opened}
        onClose={toggle}
        position="bottom"
        size="38%"
        title={t("pages.edit-recipe.ingredient.new")}
      >
        {editedIngredient && (
          <EditIngredientForm
            id={editedIngredient.id}
            existing={editedIngredient}
          />
        )}
      </Drawer>
    </>
  );
}

interface EditIngredientFormProps {
  existing: RecipeIngredient;
  id: string;
}
function EditIngredientForm({ existing, id }: EditIngredientFormProps) {
  const { t } = useTranslation();
  const form = useForm<RecipeIngredient>({
    initialValues: existing ?? {
      id,
      raw: {
        kind: "rawIngredient",
        id: 0,
        title: "",
        name: "",
        container: 0,
        proteins: 0,
        carbs: 0,
        fat: 0,
      },
    },
  });
  const onSelect = (doc: string | Document) => {
    if (typeof doc === "string") {
      form.setFieldValue("raw.name", doc);
    } else if (doc.kind !== "rawIngredient") {
      throw new Error("Invalid document type");
    } else {
      form.setValues({
        raw: {
          kind: "rawIngredient",
          id: doc.id,
          title: doc.title,
          name: doc.name,
          container: doc.container,
          proteins: doc.proteins,
          carbs: doc.carbs,
          fat: doc.fat,
        },
      });
    }
  };
  return (
    <>
      <SearchFor kind="rawIngredient" onSelect={onSelect} />
      <NumberInput
        label={t("pages.edit-recipe.ingredient.container")}
        {...form.getInputProps("raw.container")}
      />
      <Group>
        <NumberInput
          flex="1"
          label={t("pages.edit-recipe.ingredient.proteins")}
          {...form.getInputProps("raw.proteins")}
        />
        <NumberInput
          flex="1"
          label={t("pages.edit-recipe.ingredient.carbs")}
          {...form.getInputProps("raw.carbs")}
        />
        <NumberInput
          flex="1"
          label={t("pages.edit-recipe.ingredient.fat")}
          {...form.getInputProps("raw.fat")}
        />
      </Group>
    </>
  );
}
