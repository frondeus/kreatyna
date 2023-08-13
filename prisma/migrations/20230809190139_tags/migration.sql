/*
  Warnings:

  - Made the column `categoryId` on table `Tag` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "Tag" ALTER COLUMN "categoryId" SET NOT NULL;
