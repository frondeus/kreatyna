/*
  Warnings:

  - The primary key for the `ShoppingListCategory` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- DropForeignKey
ALTER TABLE "ShoppingListItem" DROP CONSTRAINT "ShoppingListItem_category_id_fkey";

-- AlterTable
ALTER TABLE "ShoppingListCategory" DROP CONSTRAINT "ShoppingListCategory_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ADD CONSTRAINT "ShoppingListCategory_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "ShoppingListCategory_id_seq";

-- AlterTable
ALTER TABLE "ShoppingListItem" ALTER COLUMN "category_id" SET DATA TYPE TEXT;

-- AddForeignKey
ALTER TABLE "ShoppingListItem" ADD CONSTRAINT "ShoppingListItem_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "ShoppingListCategory"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
