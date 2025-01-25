import { json, redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET = (async ({ locals: { session, container } }) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const service = container.rawService;
  const raw = await service.getAll();

  return json(raw);
}) satisfies RequestHandler;
