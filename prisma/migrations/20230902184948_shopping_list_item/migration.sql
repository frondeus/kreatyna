-- CreateEnum
CREATE TYPE "ShoppingListState" AS ENUM ('TODO', 'IN_CART', 'BOUGHT', 'NOT_BUING');

-- CreateTable
CREATE TABLE "ShoppingListItem" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL DEFAULT '',
    "amount" DECIMAL NOT NULL DEFAULT 0,
    "unit" TEXT,
    "state" "ShoppingListState" NOT NULL DEFAULT 'TODO',

    CONSTRAINT "ShoppingListItem_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "ShoppingListItem_id_key" ON "ShoppingListItem"("id");
