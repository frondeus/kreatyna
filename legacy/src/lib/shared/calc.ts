import type { RawIngredient } from './domains/raw/models';
import type { WithIngredients, WithRawAndAmount } from './domains/recipe/models';
import type { Nutrients } from './models';

const round2 = (num: number) => {
	return Math.round((num + Number.EPSILON) * 100) / 100;
};

export class Calculator {
	static calcKcal(nutrients: Nutrients): number {
		return round2(nutrients.proteins * 4 + nutrients.carbs * 4 + nutrients.fat * 9);
	}

	static calcNutrientsForIngredient(ingredient: WithRawAndAmount): Nutrients | undefined {
		if (!ingredient.raw) {
			return undefined;
		}

		const getPerGram = (ingredient: RawIngredient, key: keyof Nutrients): number => {
			return ingredient[key] / ingredient.container;
		};

		return {
			proteins: round2(getPerGram(ingredient.raw, 'proteins') * ingredient.amount),
			carbs: round2(getPerGram(ingredient.raw, 'carbs') * ingredient.amount),
			fat: round2(getPerGram(ingredient.raw, 'fat') * ingredient.amount)
		};
	}

	static calcNutrientsForIngredients(recipe: WithIngredients): Nutrients {
		return recipe.ingredients
			.map(
				(ingredient) =>
					Calculator.calcNutrientsForIngredient(ingredient) ?? { proteins: 0, carbs: 0, fat: 0 }
			)
			.reduce(
				(acc: Nutrients, nutrients: Nutrients) => {
					return {
						proteins: round2(acc.proteins + nutrients.proteins),
						carbs: round2(acc.carbs + nutrients.carbs),
						fat: round2(acc.fat + nutrients.fat)
					};
				},
				{ proteins: 0, carbs: 0, fat: 0 }
			);
	}
}
