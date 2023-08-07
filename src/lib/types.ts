//

import type { Raw as PrismaRaw } from '@prisma/client';
import type { Session } from 'lucia';

export type RawIngredientRow = PrismaRaw;
export interface Nutrients {
	proteins: number;
	carbs: number;
	fat: number;
}

export interface RawIngredient extends Nutrients {
	id: number;
	name: string;
	container: number;
	createdByMe: boolean | undefined;
}

// export type RawIngredientRow = Database['public']['Tables']['raw']['Row'];
// export type RawIngredientInsert = Database['public']['Tables']['raw']['Insert'];

export interface WithIngredients {
	ingredients: WithRawAndAmount[];
}

export interface Recipy {
	id: number;
	name: string;
	portions: number;
	steps: string[];
	ingredients: Ingredient[];
}

export interface WithRawAndAmount {
	raw?: RawIngredient;
	amount: number;
}

export interface Ingredient {
	id: number;
	name: string;
	unit?: string;
	amount: number;
	raw?: RawIngredient;
}

export const fromDB = {
	raw(session: Session | undefined, dbEntry: RawIngredientRow): RawIngredient {
		const by = dbEntry.created_by;
		const createdByMe = session ? by === session?.user?.userId : undefined;
		// const createdByMe = false;
		return {
			createdByMe,
			id: dbEntry.id,
			name: dbEntry.name,
			container: dbEntry.container,
			proteins: dbEntry.proteins.toNumber(),
			carbs: dbEntry.carbs.toNumber(),
			fat: dbEntry.fat.toNumber()
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
