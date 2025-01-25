/*
  Warnings:

  - You are about to drop the `Recipie` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `RecipiesIngredient` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `_RecipieToTag` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "RecipiesIngredient" DROP CONSTRAINT "RecipiesIngredient_raw_id_fkey";

-- DropForeignKey
ALTER TABLE "RecipiesIngredient" DROP CONSTRAINT "RecipiesIngredient_recipe_id_fkey";

-- DropForeignKey
ALTER TABLE "_RecipieToTag" DROP CONSTRAINT "_RecipieToTag_A_fkey";

-- DropForeignKey
ALTER TABLE "_RecipieToTag" DROP CONSTRAINT "_RecipieToTag_B_fkey";

-- DropTable
DROP TABLE "Recipie";

-- DropTable
DROP TABLE "RecipiesIngredient";

-- DropTable
DROP TABLE "_RecipieToTag";

-- CreateTable
CREATE TABLE "Recipe" (
    "id" BIGSERIAL NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT NOT NULL DEFAULT '',
    "portions" BIGINT NOT NULL DEFAULT 1,
    "steps" JSON NOT NULL DEFAULT '[]',

    CONSTRAINT "Recipe_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RecipesIngredient" (
    "id" BIGSERIAL NOT NULL,
    "recipe_id" BIGINT NOT NULL,
    "raw_id" INTEGER,
    "name" TEXT NOT NULL DEFAULT '',
    "amount" DECIMAL NOT NULL DEFAULT 0,
    "unit" TEXT,

    CONSTRAINT "RecipesIngredient_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_RecipeToTag" (
    "A" BIGINT NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Recipe_id_key" ON "Recipe"("id");

-- CreateIndex
CREATE UNIQUE INDEX "RecipesIngredient_id_key" ON "RecipesIngredient"("id");

-- CreateIndex
CREATE UNIQUE INDEX "_RecipeToTag_AB_unique" ON "_RecipeToTag"("A", "B");

-- CreateIndex
CREATE INDEX "_RecipeToTag_B_index" ON "_RecipeToTag"("B");

-- AddForeignKey
ALTER TABLE "RecipesIngredient" ADD CONSTRAINT "RecipesIngredient_raw_id_fkey" FOREIGN KEY ("raw_id") REFERENCES "Raw"("id") ON DELETE SET NULL ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "RecipesIngredient" ADD CONSTRAINT "RecipesIngredient_recipe_id_fkey" FOREIGN KEY ("recipe_id") REFERENCES "Recipe"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "_RecipeToTag" ADD CONSTRAINT "_RecipeToTag_A_fkey" FOREIGN KEY ("A") REFERENCES "Recipe"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_RecipeToTag" ADD CONSTRAINT "_RecipeToTag_B_fkey" FOREIGN KEY ("B") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE CASCADE;
