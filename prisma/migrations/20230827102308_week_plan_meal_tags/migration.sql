-- CreateTable
CREATE TABLE "_TagToWeekPlanMeal" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_TagToWeekPlanMeal_AB_unique" ON "_TagToWeekPlanMeal"("A", "B");

-- CreateIndex
CREATE INDEX "_TagToWeekPlanMeal_B_index" ON "_TagToWeekPlanMeal"("B");

-- AddForeignKey
ALTER TABLE "_TagToWeekPlanMeal" ADD CONSTRAINT "_TagToWeekPlanMeal_A_fkey" FOREIGN KEY ("A") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_TagToWeekPlanMeal" ADD CONSTRAINT "_TagToWeekPlanMeal_B_fkey" FOREIGN KEY ("B") REFERENCES "WeekPlanMeal"("id") ON DELETE CASCADE ON UPDATE CASCADE;
