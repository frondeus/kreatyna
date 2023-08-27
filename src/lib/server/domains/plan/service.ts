import { type WeekPlan, type WeekPlanMeal, weekPlan } from '$shared/domains/plan/models';
import type { PrismaClient } from '@prisma/client';
import type { RecipeService } from '../recipe/service';
import type { TagService } from '../tag/service';
import type { RecipeSummaryMap } from '../recipe/map';
import type { WeekPlanMap } from './map';
import type { RecipeSummary } from '$shared/domains/recipe/models';
import { notNull } from '$shared/utils';
import type { TagWithCategory } from '$shared/domains/tags/models';

export class WeekPlanService {
	constructor(
		readonly prisma: PrismaClient,
		readonly recipeService: RecipeService,
		readonly tagService: TagService,
		readonly recipeSummaryMap: RecipeSummaryMap,
		readonly weekPlanMap: WeekPlanMap
	) {}

	async getWeekPlan(year: number, week: number): Promise<WeekPlan | null> {
		return await this.prisma.weekPlan.findUnique({
			...weekPlan,
			where: {
				year_week: {
					year: year,
					week: week
				}
			}
		});
	}

	async generatePopulateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.generateWeekPlan(year, week);

		const populatedPlan = await this.populateWeekPlan(weekPlan);

		const createInput = this.weekPlanMap.toCreateInput(populatedPlan);

		await this.prisma.weekPlan.create({
			data: createInput
		});

		return populatedPlan;
	}

	async generateWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const forTag = async (tag: string) =>
			await this.tagService.find({ category: 'for', name: tag });

		const [forBreakfast, forSnack, forLunch, forDesert, forDinner] = await Promise.all(
			['breakfast', 'snack', 'lunch', 'desert', 'dinner'].map(forTag)
		);
		const dayIndices = [...Array(7).keys()].map((day) => day + 1);
		const meals: WeekPlanMeal[] = [
			...dayIndices.map((day) => ({
				day,
				slot: 2,
				size: 2,
				tags: [...forBreakfast],
				name: 'Breakfast',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 6,
				size: 1,
				tags: [...forSnack],
				name: 'Snack',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 10,
				size: 2,
				tags: [...forLunch],
				name: 'Lunch',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 14,
				size: 1,
				tags: [...forDesert],
				name: 'Desert',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 17,
				size: 2,
				tags: [...forDinner],
				name: 'Dinner',
				recipe: null
			}))
		];

		return {
			year,
			week,
			meals
		};
	}

	async populateWeekPlan(weekPlan: WeekPlan): Promise<WeekPlan> {
		interface ChosenRecipe {
			recipe: RecipeSummary;
			day: number;
			used: number;
		}
		const chosenRecipes: ChosenRecipe[] =
			weekPlan.meals
				?.map((meal) =>
					meal.recipe
						? {
								recipe: this.recipeSummaryMap.toDomain(meal.recipe),
								day: meal.day,
								used: 1
						  }
						: null
				)
				.filter(notNull) ?? [];
		const chosenRecipeIds: number[] = chosenRecipes.map((recipe) => recipe.recipe.id);

		const findChosenRecipe = async (tags: TagWithCategory[], forDay: number) => {
			const chosen = chosenRecipes
				.sort((a, b) => a.day - b.day)
				.find((recipe) => {
					const recipeTags = recipe.recipe.tags.map((tag) => tag.id);
					if (recipe.used >= 2) {
						return false;
					}
					if (!tags.every((tag) => recipeTags.includes(tag.id))) {
						return false;
					}
					const dayDist = Math.abs(recipe.day - forDay);
					if (dayDist >= 2) {
						return false;
					}
					return true;
				});
			if (chosen) {
				chosen.used++;
				return chosen;
			} else {
				const recipes = await this.recipeService.getNotIn(
					chosenRecipeIds,
					tags.map((tag) => tag.id)
				);
				// get random recipe
				const recipe =
					recipes.length > 0 ? recipes[Math.floor(Math.random() * recipes.length)] : null;
				if (recipe) {
					chosenRecipes.push({ recipe, used: 1, day: forDay });
					chosenRecipeIds.push(recipe.id);
					// console.log('Add', { chosenRecipeIds });
					return { recipe, used: 1 };
				}
			}
			return null;
		};

		for (const idx in weekPlan.meals) {
			const meal = weekPlan.meals[idx];
			if (meal.recipe === null) {
				const recipe = await findChosenRecipe(meal.tags, meal.day);
				meal.recipe = recipe?.recipe ? this.recipeSummaryMap.toDb(recipe.recipe) : null;
			}
			weekPlan.meals[idx] = meal;
		}

		return weekPlan;
	}
}
