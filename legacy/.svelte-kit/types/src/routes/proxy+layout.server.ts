// @ts-nocheck
import type { LayoutServerLoad } from "./$types";

export const load = async ({
  locals: { session },
}: Parameters<LayoutServerLoad>[0]) => {
  return {
    session,
  };
};
