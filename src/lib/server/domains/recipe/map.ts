import type { Recipy } from '$lib/shared/domains/recipe/models';
import type { Recipie as PrismaRecipie } from '@prisma/client';

export class RecipeMap {
	public toDomain(dbEntry: PrismaRecipie): Recipy {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name,
			portions: Number(dbEntry.portions),
			steps: dbEntry.steps as string[],
			ingredients: [] // TODO
		};
	}
}

export class RecipeSummaryMap {
	public toDomain(dbEntry: { id: bigint; name: string }): { id: number; name: string } {
		return {
			id: Number(dbEntry.id),
			name: dbEntry.name
		};
	}
}
