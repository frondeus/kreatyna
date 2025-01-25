import { z } from "zod";

export const newDocumentMetaSchema = z.object({
    title: z.string()
})

export const newShoppingListSchema = newDocumentMetaSchema.extend({
    kind: z.literal('shoppingList'),
    items: z.array(z.string())
})

export const newDocumentSchema = z.discriminatedUnion('kind', [
    newShoppingListSchema
]);


export type NewDocument = z.infer<typeof newDocumentSchema>;
export type NewShoppingList = z.infer<typeof newShoppingListSchema>;
export type NewDocumentMeta = z.infer<typeof newDocumentMetaSchema>;