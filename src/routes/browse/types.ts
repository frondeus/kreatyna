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

export function fromDB(session: Session, dbEntry: RawIngredientRow): RawIngredient {
	const by = dbEntry.created_by;
	const createdByMe = by === session.user?.id;
	return {
		createdByMe,
		...dbEntry
	};
}
