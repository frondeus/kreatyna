import type { WeekPlanMeal } from "./models";

export const sortMeals = (a: WeekPlanMeal, b: WeekPlanMeal): number => {
  if (a.day === b.day) {
    return a.slot - b.slot;
  }

  return a.day - b.day;
};
