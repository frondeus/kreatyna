import type { Nutrients } from "$lib/shared/models";
import type { AddRawSchema, EditRawSchema } from "./schema";
import type { z } from "zod";

export interface RawIngredient extends Nutrients {
  id: number;
  name: string;
  container: number;
  createdByMe: boolean | undefined;
}

export type RawIngredientCreate = z.infer<AddRawSchema>;
export type RawIngredientUpdate = z.infer<EditRawSchema>;
