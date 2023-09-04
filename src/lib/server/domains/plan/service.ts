import type {
	WeekPlanWithId,
	WeekPlan,
	WeekPlanCreate,
	WeekPlanSummary,
	WeekPlanMealWithRecipe
} from '$shared/domains/plan/models';
import type { PrismaClient } from '@prisma/client';
import type { WeekPlanMap } from './map';
import {
	selectWeekPlan,
	selectWeekPlanRecipes,
	selectWeekPlanSummary
} from '$shared/domains/plan/queries';
import type { WeekPlanGenerator } from './generator';
import type { WeekPlanPopulatorFactory } from './populator';
import type { RecipeIngredient } from '$shared/domains/recipe/models';

export class WeekPlanService {
	constructor(
		readonly prisma: PrismaClient,
		readonly weekPlanMap: WeekPlanMap,
		readonly weekPlanGenerator: WeekPlanGenerator,
		readonly weekPlanPopulator: WeekPlanPopulatorFactory
	) {}

	async getWeekPlanSummaries(): Promise<WeekPlanSummary[]> {
		return await this.prisma.weekPlan.findMany({
			...selectWeekPlanSummary,
			orderBy: [
				{
					year: 'desc'
				},
				{
					week: 'desc'
				}
			],
			take: 5
		});
	}

	async getWeekPlan(year: number, week: number): Promise<WeekPlanWithId | null> {
		return await this.prisma.weekPlan.findUnique({
			...selectWeekPlan,
			where: {
				year_week: {
					year: year,
					week: week
				}
			}
		});
	}

	async createWeekPlan(weekPlan: WeekPlanCreate): Promise<WeekPlan> {
		const createInput = this.weekPlanMap.toCreateInput(weekPlan);

		return await this.prisma.weekPlan.create({
			data: createInput,
			...selectWeekPlan
		});
	}

	async updateWeekPlan(weekPlan: WeekPlanWithId): Promise<WeekPlanWithId> {
		const updateInput = this.weekPlanMap.toUpdateInput(weekPlan);
		await this.prisma.weekPlan.update({
			data: updateInput,
			where: {
				year_week: {
					year: weekPlan.year,
					week: weekPlan.week
				}
			}
		});
		return weekPlan;
	}

	async getMeals(meals: number[]): Promise<WeekPlanMealWithRecipe[]> {
		return await this.prisma.weekPlanMeal.findMany({
			...selectWeekPlanRecipes,
			where: {
				id: {
					in: meals
				}
			}
		});
	}

	async unlinkMeals(meals: number[], year: number, week: number): Promise<void> {
		await this.prisma.weekPlanMeal.updateMany({
			data: {
				recipe_id: null
			},
			where: {
				id: {
					in: meals
				},
				weekPlan: {
					year,
					week
				}
			}
		});
	}

	async generateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.weekPlanGenerator.generateWeekPlan(year, week);
		return await this.createWeekPlan(weekPlan);
	}

	async generatePopulateAndSaveWeekPlan(year: number, week: number): Promise<WeekPlan> {
		const weekPlan = await this.weekPlanGenerator.generateWeekPlan(year, week);
		const populatedPlan = await this.weekPlanPopulator.populateWeekPlan(weekPlan);

		return await this.createWeekPlan(populatedPlan);
	}

	async populateAndUpdateWeekPlan(weekPlan: WeekPlanWithId): Promise<WeekPlanWithId> {
		const populatedPlan = await this.weekPlanPopulator.populateWeekPlan(weekPlan);

		return await this.updateWeekPlan(populatedPlan);
	}

	async groupRecipeIngredientsLinkedToMeals(meals: number[]): Promise<RecipeIngredient[]> {
		const mealPlans = await this.getMeals(meals);
		const recipes = mealPlans.flatMap((meal) => meal.recipe);
		const ingredients: RecipeIngredient[] = recipes.flatMap((r) => r?.recipesIngredients ?? []);
		const [ingredientsLinkedToRaw, ingredientsNotLinkedToRaw] = ingredients.reduce(
			(acc, ingredient) => {
				if (ingredient.raw !== null) {
					acc[0].push(ingredient);
				} else {
					acc[1].push(ingredient);
				}
				return acc;
			},
			[[], []] as [RecipeIngredient[], RecipeIngredient[]]
		);

		const groupedIngredientsLinkedToRaw = ingredientsLinkedToRaw.reduce((acc, ingredient) => {
			const key = ingredient.raw?.id.toString() + ' ' + ingredient.unit;
			const existing = acc.get(key);
			if (existing) {
				existing.amount = existing.amount.plus(ingredient.amount);
			} else {
				acc.set(key, ingredient);
			}
			return acc;
		}, new Map<string, RecipeIngredient>());

		const groupedIngredientsNotLinkedToRaw = ingredientsNotLinkedToRaw.reduce((acc, ingredient) => {
			const key = ingredient.name + ' ' + ingredient.unit;
			const existing = acc.get(key);
			if (existing) {
				existing.amount = existing.amount.plus(ingredient.amount);
			} else {
				acc.set(key, ingredient);
			}
			return acc;
		}, new Map<string, RecipeIngredient>());

		return [
			...groupedIngredientsLinkedToRaw.values(),
			...groupedIngredientsNotLinkedToRaw.values()
		];
	}
}
