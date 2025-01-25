import { json, redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET = (async ({ locals: { session, container } }) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const tag = container.tagService;
  const tags = await tag.getAll();

  return json(tags);
}) satisfies RequestHandler;
