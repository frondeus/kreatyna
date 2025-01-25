// @ts-nocheck
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = async ({
  locals: { session },
}: Parameters<LayoutServerLoad>[0]) => {
  const userId = session?.user?.userId;

  if (!userId) {
    throw error(403, "Unauthorized");
  }

  const isAdmin = session?.user?.is_admin ?? false;

  if (!isAdmin) {
    throw error(403, "Unauthorized");
  }

  return {};
};
