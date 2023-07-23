import type { Database } from '$lib/database.types';
import type { Session } from '@supabase/supabase-js';

export interface RawIngredient {
	id: number;
	name: string;
	container: number;
	proteins: number;
	carbs: number;
	fat: number;
	createdByMe: boolean;
}

export type RawIngredientRow = Database['public']['Tables']['raw']['Row'];

export interface Recipy {
	id: number,
	name: string,
	portions: number,
	steps: string[],
	ingredients: Ingredient[]
}

export interface Ingredient {
	id: number,
	name: string,
	raw?: RawIngredient,
	amount: number,
	unit?: string
}

export const fromDB = {
	raw(session: Session, dbEntry: RawIngredientRow): RawIngredient {
		
		const by = dbEntry.created_by;
		const createdByMe = by === session.user?.id;
		return {
			createdByMe,
			...dbEntry
		};
	}
};
