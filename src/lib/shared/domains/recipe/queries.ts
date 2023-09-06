import { Prisma } from '@prisma/client';
import { tagWithCategory, tagWithCategoryName } from '../tags/queries';

export const ingredients = Prisma.validator<Prisma.RecipesIngredientFindManyArgs>()({
	select: {
		id: true,
		amount: true,
		unit: true,
		name: true,
		recipe_id: true,
		raw_id: true,
		raw: {
			select: {
				id: true,
				name: true,
				created_by: true,
				container: true,
				proteins: true,
				carbs: true,
				fat: true,
				tagId: true
			}
		}
	}
});

export const recipeSummarySelect = Prisma.validator<Prisma.RecipeFindManyArgs>()({
	select: {
		id: true,
		name: true,
		tags: tagWithCategoryName
	}
});

export const ingredientsWithRecipes = Prisma.validator<Prisma.RecipesIngredientFindManyArgs>()({
	select: {
		...ingredients.select,
		recipes: recipeSummarySelect
	}
});

export const recipeSelect = Prisma.validator<Prisma.RecipeFindManyArgs>()({
	include: {
		recipesIngredients: {
			include: {
				raw: true
			}
		},
		tags: tagWithCategory
	}
});

export const recipeWithIngredients = Prisma.validator<Prisma.RecipeFindManyArgs>()({
	select: {
		id: true,
		name: true,
		tags: tagWithCategoryName,
		recipesIngredients: ingredients
	}
});
