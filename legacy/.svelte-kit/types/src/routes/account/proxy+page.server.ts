// @ts-nocheck
import { auth } from "$lib/server/lucia";
import { fail, redirect, type Actions, type RequestEvent } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms/server";

import { z } from "zod";

const updateSchema = z.object({
  fullName: z.string().default("John Doe").optional(),
});

const signoutSchema = z.object({});

export const load = async ({ locals: { session, prisma } }: RequestEvent) => {
  if (!session) {
    throw redirect(303, "/");
  }

  const user = await prisma.user.findUnique({
    where: {
      id: session.user.userId,
    },
  });

  const updateForm = await superValidate(
    {
      fullName: user?.full_name ?? undefined,
    },
    updateSchema,
  );
  const signoutForm = await superValidate(signoutSchema);

  return { session, updateForm, signoutForm };
};

export const actions = {
  update: async ({
    request,
    locals: { session, prisma },
  }: import("./$types").RequestEvent) => {
    const form = await superValidate(request, updateSchema);

    if (!form.valid) {
      return fail(400, { form });
    }

    if (!session) {
      return redirect(303, "/login");
    }

    await prisma.user.update({
      where: {
        id: session.user.userId,
      },
      data: {
        full_name: form.data.fullName,
      },
    });

    return {
      form,
    };
  },
  signout: async ({
    locals: { session, setSession },
  }: import("./$types").RequestEvent) => {
    if (session) {
      await auth.invalidateSession(session.sessionId);
      setSession(null);
      throw redirect(303, "/login");
    }
  },
};
null as any as Actions;
