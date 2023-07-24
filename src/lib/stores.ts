import { readable, type Readable } from 'svelte/store';
import type { Database } from './database.types';
import type { SupabaseClient } from '@supabase/supabase-js';

// export type RawIngredentsStore = Writable<RawIngredient[]>;
// export type IngredientsStore = Readable<Ingredient[]>;
// export type SumStore = Readable<Readable<number>>;

export function realtime<T extends { id: K }, K, R>(
	supabase: SupabaseClient<Database>,
	init: T[],
	table: string,
	map: (arg0: R) => T
): Readable<T[]> {
	return readable(init, (_set, update) => {
		const channel = supabase
			.channel('realtime_' + table)
			.on(
				'postgres_changes',
				{
					event: '*',
					schema: 'public',
					table
				},
				(payload) => {
					update((current) => {
						switch (payload.eventType) {
							case 'INSERT':
								if (current.findIndex((row: T) => row.id === payload.new.id) === -1) {
									return [...current, map(payload.new as R)];
								} else {
									return current;
								}
							case 'DELETE':
								return current.filter((row: T) => row.id !== payload.old.id);
							case 'UPDATE': {
								const idx = current.findIndex((row: T) => row.id === payload.old.id);
								return [...current.slice(0, idx), map(payload.new as R), ...current.slice(idx + 1)];
							}
						}
					});
				}
			)
			.subscribe();

		return () => {
			channel.unsubscribe();
		};
	});
}
// //-----
// export function derivedSum(
// 	ingredients: IngredientsStore,
// 	extractor: (data: IngredientWithData) => number
// ): SumStore {
// 	return derived(ingredients, (ingredients) => {
// 		const sums = ingredients.map((id: Ingredient) => id.data);
// 		return derived(sums, (sums) => sums.reduce((acc: number, data) => acc + extractor(data), 0), 0);
// 	});
// }

// export function derivedIngredient(
// 	ingredient: Writable<number>,
// 	raw: RawIngredient
// ): Readable<IngredientWithData> {
// 	return derived(
// 		[ingredient, raw.name, raw.container, raw.protein, raw.carbs, raw.fat],
// 		([amount, name, container, protein, carbs, fat]) => {
// 			return {
// 				name,
// 				amount: ingredient,
// 				container,
// 				protein: (protein / container) * amount,
// 				carbs: (carbs / container) * amount,
// 				fat: (fat / container) * amount
// 			};
// 		}
// 	);
// }
