import { superValidate } from "sveltekit-superforms/server";
import type { PageServerLoad } from "./$types";
import { addRawSchema } from "$lib/shared/domains/raw/schema";
import { redirect, type Actions, fail } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals: { session } }) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const form = superValidate(undefined, addRawSchema);

  return { form, session };
};

export const actions: Actions = {
  default: async ({ request, locals: { session, container } }) => {
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
