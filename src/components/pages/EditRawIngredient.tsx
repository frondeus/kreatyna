import { APIContext } from "@/lib/api";
import { RawIngredient } from "@/lib/types";
import { onScalarChange, useSubscribeForm } from "@/lib/utils";
import { Fieldset, NumberInput, Paper, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { useTranslation } from "react-i18next";

export interface EditRawIngredientProps {
  id: string;
}

export default function EditRawIngredient({ id }: EditRawIngredientProps) {
  const { t } = useTranslation();
  const api = useContext(APIContext);
  const { data } = useSuspenseQuery({
    queryKey: ["raw-ingredient", id],
    queryFn: () => api.getDocument(parseInt(id)),
  });

  if (data.kind !== "rawIngredient") {
    throw new Error("Document is not a raw ingredient");
  }

  const form = useForm<RawIngredient>({
    initialValues: data,
  });
  useSubscribeForm(data.id, api, form);

  return (
    <Paper>
      <Fieldset legend={t("pages.edit-raw-ingredient.legend")}>
        <TextInput
          label={t("pages.edit-raw-ingredient.name")}
          {...form.getInputProps("name")}
          onChange={onScalarChange(form, api, "name")}
        />

        <NumberInput
          label={t("pages.edit-raw-ingredient.container")}
          {...form.getInputProps("container")}
          onChange={onScalarChange(form, api, "container")}
        />
      </Fieldset>
      <Fieldset legend={t("pages.edit-raw-ingredient.nutrition")}>
        <NumberInput
          label={t("pages.edit-raw-ingredient.proteins")}
          {...form.getInputProps("proteins")}
          onChange={onScalarChange(form, api, "proteins")}
        />
        <NumberInput
          label={t("pages.edit-raw-ingredient.carbs")}
          {...form.getInputProps("carbs")}
          onChange={onScalarChange(form, api, "carbs")}
        />
        <NumberInput
          label={t("pages.edit-raw-ingredient.fat")}
          {...form.getInputProps("fat")}
          onChange={onScalarChange(form, api, "fat")}
        />
      </Fieldset>
    </Paper>
  );
}
