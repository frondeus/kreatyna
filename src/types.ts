import type { Readable, Writable } from 'svelte/store';

export function round2(num: number, places = 2) {
	const pl = places;
	return +(Math.round((num + 'e+' + pl) as unknown as number) + 'e-' + places);
}

export interface RawIngredient {
	name: Writable<string>;
	container: Writable<number>;
	protein: Writable<number>;
	carbs: Writable<number>;
	fat: Writable<number>;
}

export interface Ingredient {
	amount: Writable<number>;
	data: Readable<IngredientWithData>;
}

export interface IngredientWithData {
	amount: Writable<number>;
	name: string;
	container: number;
	protein: number;
	carbs: number;
	fat: number;
}

export interface Goal {
	protein: Writable<number>;
	carbs: Writable<number>;
	fat: Writable<number>;
}

export interface Sum {
	protein: Readable<number>;
	carbs: Readable<number>;
	fat: Readable<number>;
}
