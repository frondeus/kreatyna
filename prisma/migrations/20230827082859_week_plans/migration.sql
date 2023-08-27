-- CreateTable
CREATE TABLE "WeekPlan" (
    "year" INTEGER NOT NULL,
    "week" INTEGER NOT NULL,

    CONSTRAINT "WeekPlan_pkey" PRIMARY KEY ("year","week")
);

-- CreateTable
CREATE TABLE "WeekPlanMeal" (
    "id" SERIAL NOT NULL,
    "year" INTEGER NOT NULL,
    "week" INTEGER NOT NULL,
    "day" INTEGER NOT NULL,
    "slot" INTEGER NOT NULL,
    "size" INTEGER NOT NULL,
    "recipe_id" BIGINT,

    CONSTRAINT "WeekPlanMeal_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "WeekPlanMeal_id_key" ON "WeekPlanMeal"("id");

-- AddForeignKey
ALTER TABLE "WeekPlanMeal" ADD CONSTRAINT "WeekPlanMeal_year_week_fkey" FOREIGN KEY ("year", "week") REFERENCES "WeekPlan"("year", "week") ON DELETE CASCADE ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "WeekPlanMeal" ADD CONSTRAINT "WeekPlanMeal_recipe_id_fkey" FOREIGN KEY ("recipe_id") REFERENCES "Recipe"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
