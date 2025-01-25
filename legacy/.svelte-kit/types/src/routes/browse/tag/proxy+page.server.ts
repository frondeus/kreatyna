// @ts-nocheck
import { redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = async ({
  locals: { session, container },
}: Parameters<PageServerLoad>[0]) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const tag = container.tagService;

  return {
    session,
    tags: tag.getAll(),
  };
};

export const actions = {
  delete: async ({
    request,
    locals: { session, container },
  }: import("./$types").RequestEvent) => {
    if (!session) {
      throw redirect(303, "/login");
    }

    const data = await request.formData();
    const id = Number(data.get("id"));

    const tag = container.tagService;
    await tag.remove(id);

    return {};
  },
};
null as any as Actions;
