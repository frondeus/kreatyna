import { Prisma } from '@prisma/client';
import { tagWithCategory, tagWithCategoryName } from '../tags/queries';

export const ingredients = Prisma.validator<Prisma.RecipesIngredientFindManyArgs>()({
	select: {
		id: true,
		amount: true,
		unit: true,
		name: true,
		raw: {
			select: {
				id: true,
				name: true
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
		recipesIngredients: ingredients
	}
});
