/*
  Warnings:

  - You are about to drop the `TagsOnRaws` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `TagsOnRecipies` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "TagsOnRaws" DROP CONSTRAINT "TagsOnRaws_raw_id_fkey";

-- DropForeignKey
ALTER TABLE "TagsOnRaws" DROP CONSTRAINT "TagsOnRaws_tag_id_fkey";

-- DropForeignKey
ALTER TABLE "TagsOnRecipies" DROP CONSTRAINT "TagsOnRecipies_recipie_id_fkey";

-- DropForeignKey
ALTER TABLE "TagsOnRecipies" DROP CONSTRAINT "TagsOnRecipies_tag_id_fkey";

-- DropTable
DROP TABLE "TagsOnRaws";

-- DropTable
DROP TABLE "TagsOnRecipies";

-- CreateTable
CREATE TABLE "_RawToTag" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE "_RecipieToTag" (
    "A" BIGINT NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_RawToTag_AB_unique" ON "_RawToTag"("A", "B");

-- CreateIndex
CREATE INDEX "_RawToTag_B_index" ON "_RawToTag"("B");

-- CreateIndex
CREATE UNIQUE INDEX "_RecipieToTag_AB_unique" ON "_RecipieToTag"("A", "B");

-- CreateIndex
CREATE INDEX "_RecipieToTag_B_index" ON "_RecipieToTag"("B");

-- AddForeignKey
ALTER TABLE "_RawToTag" ADD CONSTRAINT "_RawToTag_A_fkey" FOREIGN KEY ("A") REFERENCES "Raw"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_RawToTag" ADD CONSTRAINT "_RawToTag_B_fkey" FOREIGN KEY ("B") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_RecipieToTag" ADD CONSTRAINT "_RecipieToTag_A_fkey" FOREIGN KEY ("A") REFERENCES "Recipie"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_RecipieToTag" ADD CONSTRAINT "_RecipieToTag_B_fkey" FOREIGN KEY ("B") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE CASCADE;
