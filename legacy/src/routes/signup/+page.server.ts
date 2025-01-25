import { auth } from "$lib/server/lucia";
import { fail, redirect, error } from "@sveltejs/kit";

import type { Actions, PageServerLoad } from "./$types";
import { z } from "zod";
import { superValidate } from "sveltekit-superforms/server";

const signupSchema = z
  .object({
    username: z.string().min(3).max(32),
    password: z.string().min(8).max(64),
    passwordConfirm: z.string().min(8).max(64),
  })
  .refine(({ password, passwordConfirm }) => password === passwordConfirm, {
    message: "Passwords do not match.",
    path: ["passwordConfirm"],
  });

export const load = (async ({ locals: { session } }) => {
  throw redirect(302, "/login");

  if (session) {
    throw redirect(302, "/");
  }
  const form = superValidate(signupSchema);
  return { form };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async ({ request, locals }) => {
    throw redirect(302, "/login");

    const form = await superValidate(request, signupSchema);
    if (!form.valid) {
      return fail(400, { form });
    }

    try {
      const user = await auth.createUser({
        key: {
          providerId: "username",
          providerUserId: form.data.username.toLowerCase(),
          password: form.data.password,
        },
        attributes: {
          username: form.data.username,
          is_admin: false,
          full_name: "",
        },
      });
      const session = await auth.createSession({
        userId: user.userId,
        attributes: {},
      });
      locals.setSession(session);
    } catch (e) {
      throw error(500, { message: "An unknown error occurred." });
    }

    throw redirect(302, "/");
  },
};
