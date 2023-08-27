/*
  Warnings:

  - Added the required column `name` to the `WeekPlanMeal` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "WeekPlanMeal" ADD COLUMN     "name" TEXT NOT NULL;
