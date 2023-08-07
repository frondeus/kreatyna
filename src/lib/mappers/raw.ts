import type { RawIngredient } from '$lib/types';
import type { Raw as PrismaRaw } from '@prisma/client';
import { Decimal } from '@prisma/client/runtime/library';
import type { Session } from 'lucia';

export class RawMap {
	session: Session | null;

	constructor(session: Session | null) {
		this.session = session;
	}

	public toDomain(dbEntry: PrismaRaw): RawIngredient {
		const by = dbEntry.created_by;
		const createdByMe = this.session ? by === this.session?.user?.userId : undefined;
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

	public toDB(domain: RawIngredient): PrismaRaw {
		return {
			id: domain.id,
			name: domain.name,
			container: domain.container,
			proteins: new Decimal(domain.proteins),
			carbs: new Decimal(domain.carbs),
			fat: new Decimal(domain.fat),
			created_by: this.session?.user?.userId ?? null
		};
	}
}
