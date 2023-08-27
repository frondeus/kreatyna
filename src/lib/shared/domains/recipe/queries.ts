import { Prisma } from '@prisma/client';
import { tagWithCategory, tagWithCategoryName } from '../tags/queries';

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
