// @ts-nocheck
import { superValidate } from "sveltekit-superforms/server";
import type { PageServerLoad } from "./$types";
import {
  fail,
  redirect,
  error as returnError,
  type Actions,
} from "@sveltejs/kit";
import { notNullOrUndefined } from "$lib/shared/utils";
import {
  editRecipeSchema,
  type EditRecipeSchema,
} from "$lib/shared/domains/recipe/schema";
import type { SuperValidated } from "sveltekit-superforms";

export const load = async ({
  params,
  locals: { session, container },
}: Parameters<PageServerLoad>[0]) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const id = Number(params.id);

  const recipe = container.recipeService;
  const data = await recipe.getOne(id);

  if (data === null) {
    throw returnError(404, "Recipe was not found");
  }

  const ingredients = data.ingredients ?? [];
  const relatedIngredients = new Map(
    ingredients
      .map((i) => i.raw)
      .filter(notNullOrUndefined)
      .map((r) => [r.id, r] as const),
  );

  const formData = recipe.map.toFormSchema(data);

  const form: SuperValidated<EditRecipeSchema> = await superValidate(
    formData,
    editRecipeSchema,
  );

  return { session, form, relatedIngredients };
};

export const actions = {
  default: async ({
    request,
    locals: { session, container },
  }: import("./$types").RequestEvent) => {
    if (!session) {
      throw redirect(303, "/login");
    }

    const form = await superValidate(request, editRecipeSchema);

    if (!form.valid) {
      return fail(400, { form });
    }

    const recipe = container.recipeService;
    await recipe.update(form.data);

    return { form };
  },
};
null as any as Actions;
