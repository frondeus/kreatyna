import { UpdateShoppingListItemWithCategorySchema } from "$shared/domains/shopping/schema";
import { error, json, redirect, type RequestHandler } from "@sveltejs/kit";

const BodySchema = UpdateShoppingListItemWithCategorySchema.array();

export const PUT = (async ({ request, locals: { session, container } }) => {
  if (!session) {
    throw redirect(303, "/login");
  }

  const body = await request.json();
  const validatedBody = BodySchema.safeParse(body);
  if (validatedBody.success === false) {
    throw error(400, validatedBody.error.message);
  }

  const service = container.shoppingListService;

  for (const schema of validatedBody.data) {
    // TODO: Currently prisma is not very good at returning exception when updating a non-existing item.
    // It's easier to check if the item exists before updating it.
    const item = await service.getShoppingListItem(schema.id);
    if (item === null) {
      throw error(404, "Shopping list item not found");
    }

    await service.updateShoppingListItem(schema);
  }

  return json("ok");
}) satisfies RequestHandler;
