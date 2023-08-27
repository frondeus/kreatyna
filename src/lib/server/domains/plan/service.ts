import type { WeekPlan, WeekPlanMeal } from "$shared/domains/plan/models";
import type { PrismaClient } from "@prisma/client";
import type { RecipeService } from "../recipe/service";
import type { TagService } from "../tag/service";
import type { RecipeSummaryMap } from "../recipe/map";

export class WeekPlanService {
    prisma: PrismaClient;
    recipeService: RecipeService;
    tagService: TagService;
    recipeSummaryMap: RecipeSummaryMap;

    constructor(prisma: PrismaClient, recipeService: RecipeService, tagService: TagService, recipeSummaryMap: RecipeSummaryMap) {
        this.prisma = prisma;
        this.recipeService = recipeService;
        this.tagService = tagService;
        this.recipeSummaryMap = recipeSummaryMap;
    }

    async getWeekPlan(year: number, week: number): Promise<WeekPlan | null> {
        return await this.prisma.weekPlan.findUnique({
            include: {
                meals: {
                    include: {
                        recipe: {
                            include: {
                                tags: true
                            }
                        },
                    }
                }
            },
            where: {
                year_week: {
                    year: year,
                    week: week
                }
            }
        });
    }

    async generateWeekPlan(year: number, week: number): Promise<WeekPlan> {
        const dayIndices = [...Array(7).keys()]
            .map((day) => day + 1)
            .filter((day) => day % 2 === 1);

        const getRecipiesFor = async (tags: string[]) => {
            const getForTag = async (tag: string) => {
                const forTag = await this.tagService.findIds({ category: 'for', name: tag });
                const recipies = await this.recipeService.getAll(forTag);
                return [forTag, recipies] as const;
            };
            const results = await Promise.all(tags.map(getForTag));

            return results.map(([forTag, recipies], idx) => {
                const others = results.slice(0, idx).concat(results.slice(idx + 1)).map(([, recipies]) => recipies);
                const takeOneFrom = () => {
                    const chosen = recipies.splice(Math.floor(Math.random() * recipies.length), 1)[0];
                    others.forEach(other => {
                        const index = other.findIndex(recipe => recipe.id === chosen.id);
                        if (index > -1) {
                            other.splice(index, 1);
                        }
                    });
                    return this.recipeSummaryMap.toDb(chosen);
                };
                return [forTag, takeOneFrom] as const;
            });
        };

        const [
            [forBreakfast, breakfast],
            [forSnack, snack],
            [forLunch, lunch],
            [forDinner, dinner],
            [forDesert, desert]
        ] = await getRecipiesFor(['breakfast', 'snack', 'lunch', 'dinner', 'desert']);

        const meals: WeekPlanMeal[] = [
            ...dayIndices.map((day) => ({
                day,
                slot: 2,
                size: 2,
                tags: [...forBreakfast],
                name: 'Breakfast',
                recipe: breakfast()
            })),
            ...dayIndices.map((day) => ({
                day,
                slot: 6,
                size: 1,
                tags: [...forSnack],
                name: 'Snack',
                recipe: snack()
            })),
            ...dayIndices.map((day) => ({
                day,
                slot: 10,
                size: 2,
                tags: [...forLunch],
                name: 'Lunch',
                recipe: lunch()
            })),
            ...dayIndices.map((day) => ({
                day,
                slot: 14,
                size: 1,
                tags: [...forDesert],
                name: 'Desert',
                recipe: desert()
            })),
            ...dayIndices.map((day) => ({
                day,
                slot: 17,
                size: 2,
                tags: [...forDinner],
                name: 'Dinner',
                recipe: dinner()
            }))
        ]
            .flatMap(meal => {
                if (meal.day < 7) {
                    return [meal, { ...meal, day: meal.day + 1 }];
                } else return [meal]
            })
            ;

        return {
            year,
            week,
            meals
        }
    }
}