-- CreateTable
CREATE TABLE "_RecipesIngredientToShoppingListItem" (
    "A" BIGINT NOT NULL,
    "B" UUID NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_RecipesIngredientToShoppingListItem_AB_unique" ON "_RecipesIngredientToShoppingListItem"("A", "B");

-- CreateIndex
CREATE INDEX "_RecipesIngredientToShoppingListItem_B_index" ON "_RecipesIngredientToShoppingListItem"("B");

-- AddForeignKey
ALTER TABLE "_RecipesIngredientToShoppingListItem" ADD CONSTRAINT "_RecipesIngredientToShoppingListItem_A_fkey" FOREIGN KEY ("A") REFERENCES "RecipesIngredient"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_RecipesIngredientToShoppingListItem" ADD CONSTRAINT "_RecipesIngredientToShoppingListItem_B_fkey" FOREIGN KEY ("B") REFERENCES "ShoppingListItem"("id") ON DELETE CASCADE ON UPDATE CASCADE;
