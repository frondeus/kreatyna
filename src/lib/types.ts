//
import type { Database } from '$lib/database.types';
import type { Session } from '@supabase/supabase-js';

export interface RawIngredient {
	id: number;
	name: string;
	container: number;
	proteins: number;
	carbs: number;
	fat: number;
	createdByMe: boolean | undefined;
}

export type RawIngredientRow = Database['public']['Tables']['raw']['Row'];

export interface Recipy {
	id: number;
	name: string;
	portions: number;
	steps: string[];
	ingredients: Ingredient[];
}

export interface Ingredient {
	id: number;
	name: string;
	raw?: RawIngredient;
	amount: number;
	unit?: string;
}

export const fromDB = {
	raw(session: Session | undefined, dbEntry: RawIngredientRow): RawIngredient {
		const by = dbEntry.created_by;
		const createdByMe = session ? by === session?.user?.id : undefined;
		return {
			createdByMe,
			...dbEntry
		};
	}
};

// import type { Readable, Writable } from 'svelte/store';

// export function round2(num: number, places = 2) {
// 	const pl = places;
// 	return +(Math.round((num + 'e+' + pl) as unknown as number) + 'e-' + places);
// }

// export interface RawIngredient {
// 	name: Writable<string>;
// 	container: Writable<number>;
// 	protein: Writable<number>;
// 	carbs: Writable<number>;
// 	fat: Writable<number>;
// }

// export interface Ingredient {
// 	amount: Writable<number>;
// 	data: Readable<IngredientWithData>;
// }

// export interface IngredientWithData {
// 	amount: Writable<number>;
// 	name: string;
// 	container: number;
// 	protein: number;
// 	carbs: number;
// 	fat: number;
// }

// export interface Goal {
// 	protein: Writable<number>;
// 	carbs: Writable<number>;
// 	fat: Writable<number>;
// }

// export interface Sum {
// 	protein: Readable<number>;
// 	carbs: Readable<number>;
// 	fat: Readable<number>;
// }
