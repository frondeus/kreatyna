-- AlterTable
ALTER TABLE "ShoppingListItem" ADD COLUMN     "category_id" INTEGER;

-- CreateTable
CREATE TABLE "ShoppingListCategory" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "icon" TEXT NOT NULL,
    "order" SERIAL NOT NULL,

    CONSTRAINT "ShoppingListCategory_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "ShoppingListCategory_id_key" ON "ShoppingListCategory"("id");

-- CreateIndex
CREATE UNIQUE INDEX "ShoppingListCategory_name_key" ON "ShoppingListCategory"("name");

-- AddForeignKey
ALTER TABLE "ShoppingListItem" ADD CONSTRAINT "ShoppingListItem_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "ShoppingListCategory"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
