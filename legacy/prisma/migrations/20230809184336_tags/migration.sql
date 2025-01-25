-- AlterTable
ALTER TABLE "Raw" ADD COLUMN     "tagId" INTEGER;

-- AlterTable
ALTER TABLE "Recipie" ADD COLUMN     "tagId" INTEGER;

-- CreateTable
CREATE TABLE "TagCategory" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL DEFAULT '',

    CONSTRAINT "TagCategory_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Tag" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL DEFAULT '',
    "categoryId" INTEGER,

    CONSTRAINT "Tag_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "TagCategory_id_key" ON "TagCategory"("id");

-- CreateIndex
CREATE UNIQUE INDEX "Tag_id_key" ON "Tag"("id");

-- AddForeignKey
ALTER TABLE "Raw" ADD CONSTRAINT "Raw_tagId_fkey" FOREIGN KEY ("tagId") REFERENCES "Tag"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "Recipie" ADD CONSTRAINT "Recipie_tagId_fkey" FOREIGN KEY ("tagId") REFERENCES "Tag"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "Tag" ADD CONSTRAINT "Tag_categoryId_fkey" FOREIGN KEY ("categoryId") REFERENCES "TagCategory"("id") ON DELETE CASCADE ON UPDATE NO ACTION;
