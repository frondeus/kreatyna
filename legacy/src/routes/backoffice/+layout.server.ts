import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals: { session } }) => {
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
