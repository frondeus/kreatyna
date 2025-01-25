/*
  Warnings:

  - You are about to drop the column `tagId` on the `Recipie` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[name,categoryId]` on the table `Tag` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[name]` on the table `TagCategory` will be added. If there are existing duplicate values, this will fail.

*/
-- DropForeignKey
ALTER TABLE "Raw" DROP CONSTRAINT "Raw_tagId_fkey";

-- DropForeignKey
ALTER TABLE "Recipie" DROP CONSTRAINT "Recipie_tagId_fkey";

-- AlterTable
ALTER TABLE "Recipie" DROP COLUMN "tagId";

-- CreateTable
CREATE TABLE "TagsOnRecipies" (
    "recipie_id" BIGINT NOT NULL,
    "tag_id" INTEGER NOT NULL,

    CONSTRAINT "TagsOnRecipies_pkey" PRIMARY KEY ("recipie_id","tag_id")
);

-- CreateTable
CREATE TABLE "TagsOnRaws" (
    "raw_id" INTEGER NOT NULL,
    "tag_id" INTEGER NOT NULL,

    CONSTRAINT "TagsOnRaws_pkey" PRIMARY KEY ("raw_id","tag_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Tag_name_categoryId_key" ON "Tag"("name", "categoryId");

-- CreateIndex
CREATE UNIQUE INDEX "TagCategory_name_key" ON "TagCategory"("name");

-- AddForeignKey
ALTER TABLE "TagsOnRecipies" ADD CONSTRAINT "TagsOnRecipies_recipie_id_fkey" FOREIGN KEY ("recipie_id") REFERENCES "Recipie"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "TagsOnRecipies" ADD CONSTRAINT "TagsOnRecipies_tag_id_fkey" FOREIGN KEY ("tag_id") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "TagsOnRaws" ADD CONSTRAINT "TagsOnRaws_raw_id_fkey" FOREIGN KEY ("raw_id") REFERENCES "Raw"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "TagsOnRaws" ADD CONSTRAINT "TagsOnRaws_tag_id_fkey" FOREIGN KEY ("tag_id") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE NO ACTION;
