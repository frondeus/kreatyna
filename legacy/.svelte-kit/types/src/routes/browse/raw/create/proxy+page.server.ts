// @ts-nocheck
import { superValidate } from "sveltekit-superforms/server";
import type { PageServerLoad } from "./$types";
import { addRawSchema } from "$lib/shared/domains/raw/schema";
import { redirect, type Actions, fail } from "@sveltejs/kit";

export const load = async ({
  locals: { session },
}: Parameters<PageServerLoad>[0]) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const form = superValidate(undefined, addRawSchema);

  return { form, session };
};

export const actions = {
  default: async ({
    request,
    locals: { session, container },
  }: import("./$types").RequestEvent) => {
    if (!session) {
      throw redirect(303, "/login");
    }

    const form = await superValidate(request, addRawSchema);

    if (!form.valid) {
      return fail(400, { form });
    }

    const raw = container.rawService;
    await raw.create(form.data);

    throw redirect(303, "/browse/raw");
  },
};
null as any as Actions;
