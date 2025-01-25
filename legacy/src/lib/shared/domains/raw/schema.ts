import { z } from "zod";

export const editRawSchema = z.object({
  id: z.number().int(),
  name: z.string().default(""),
  container: z.number().default(100),
  proteins: z.number().default(0),
  carbs: z.number().default(0),
  fat: z.number().default(0),
});

export type EditRawSchema = typeof editRawSchema;

export const addRawSchema = editRawSchema.omit({
  id: true,
});

export type AddRawSchema = typeof addRawSchema;
