import { derived, type Readable, type Writable } from 'svelte/store';
import type { Ingredient, IngredientWithData, RawIngredient } from './types';

export type RawIngredentsStore = Writable<RawIngredient[]>;
export type IngredientsStore = Readable<Ingredient[]>;
export type SumStore = Readable<Readable<number>>;

//-----
export function derivedSum(
	ingredients: IngredientsStore,
	extractor: (data: IngredientWithData) => number
): SumStore {
	return derived(ingredients, (ingredients) => {
		const sums = ingredients.map((id: Ingredient) => id.data);
		return derived(sums, (sums) => sums.reduce((acc: number, data) => acc + extractor(data), 0), 0);
	});
}

export function derivedIngredient(
	ingredient: Writable<number>,
	raw: RawIngredient
): Readable<IngredientWithData> {
	return derived(
		[ingredient, raw.name, raw.container, raw.protein, raw.carbs, raw.fat],
		([amount, name, container, protein, carbs, fat]) => {
			return {
				name,
				amount: ingredient,
				container,
				protein: (protein / container) * amount,
				carbs: (carbs / container) * amount,
				fat: (fat / container) * amount
			};
		}
	);
}
