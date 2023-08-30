import { Lifecycle, inject, injectable, scoped } from 'tsyringe';
import type { TagService } from '../tag/service';
import type { WeekPlanCreate } from '$shared/domains/plan/models';
import type { RecipeSummary } from '$shared/domains/recipe/models';
import type { RecipeSummaryMap } from '../recipe/map';
import { notNull } from '$shared/utils';
import type { TagWithCategory } from '$shared/domains/tags/models';
import type { RecipeService } from '../recipe/service';
import type { ExistingRecipeSelector } from './selector';

export interface ChosenRecipe {
	recipe: RecipeSummary;
	day: number;
	used: number;
}

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanPopulator {
	private chosenRecipes: ChosenRecipe[];

	private get chosenRecipeIds(): number[] {
		return this.chosenRecipes.map((recipe) => recipe.recipe.id);
	}

	constructor(
		@inject('tagService') readonly tagService: TagService,
		@inject('recipeSummaryMap') readonly recipeSummaryMap: RecipeSummaryMap,
		@inject('recipeService') readonly recipeService: RecipeService,
		@inject('existingRecipeSelector') readonly existingRecipeSelector: ExistingRecipeSelector,
		readonly weekPlan: WeekPlanCreate
	) {
		const chosen = (weekPlan.meals ?? [])
			.map((meal) => (meal.recipe ? ([meal.recipe, meal.day] as const) : null))
			.filter(notNull)
			.map(([recipe, day]) => ({
				recipe: this.recipeSummaryMap.toDomain(recipe),
				day,
				used: 1
			}))
			.reduce((acc, cur) => {
				const existing = acc.find((chosen) => chosen.recipe.id === cur.recipe.id);
				if (existing) {
					existing.used += 1;
					return acc;
				}
				return [...acc, cur];
			}, [] as ChosenRecipe[]);

		this.chosenRecipes = chosen;
	}

	private selectChosenRecipe(tags: TagWithCategory[], forDay: number): ChosenRecipe | null {
		return (
			this.chosenRecipes
				.sort((a, b) => a.day - b.day)
				.find((recipe) => this.existingRecipeSelector.selectChosenRecipe(recipe, tags, forDay)) ??
			null
		);
	}

	async findChosenRecipeOrNew(
		tags: TagWithCategory[],
		forDay: number
	): Promise<ChosenRecipe | null> {
		const chosen = this.selectChosenRecipe(tags, forDay);

		if (chosen) {
			chosen.used++;
			return chosen;
		}

		const recipes = await this.recipeService.getNotIn(
			this.chosenRecipeIds,
			tags.map((tag) => tag.id)
		);

		const recipe = recipes.length > 0 ? recipes[Math.floor(Math.random() * recipes.length)] : null;
		if (!recipe) {
			return null;
		}

		this.chosenRecipes.push({ recipe, used: 1, day: forDay });
		return { recipe, used: 1, day: forDay };
	}
}

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanPopulatorFactory {
	constructor(
		@inject('tagService') readonly tagService: TagService,
		@inject('recipeSummaryMap') readonly recipeSummaryMap: RecipeSummaryMap,
		@inject('recipeService') readonly recipeService: RecipeService,
		@inject('existingRecipeSelector') readonly existingRecipeSelector: ExistingRecipeSelector
	) {}

	forWeekPlan(weekPlan: WeekPlanCreate): WeekPlanPopulator {
		return new WeekPlanPopulator(
			this.tagService,
			this.recipeSummaryMap,
			this.recipeService,
			this.existingRecipeSelector,
			weekPlan
		);
	}

	async populateWeekPlan<W extends WeekPlanCreate>(weekPlan: W): Promise<W> {
		const populator = this.forWeekPlan(weekPlan);

		for (const idx in weekPlan.meals) {
			const meal = weekPlan.meals[idx];
			if (meal.recipe === null) {
				const recipe = await populator.findChosenRecipeOrNew(meal.tags, meal.day);
				meal.recipe = recipe?.recipe ? this.recipeSummaryMap.toDb(recipe.recipe) : null;
			}
			weekPlan.meals[idx] = meal;
		}

		return weekPlan;
	}
}
