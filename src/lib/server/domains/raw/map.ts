import type {
	RawIngredient,
	RawIngredientCreate,
	RawIngredientUpdate
} from '$lib/shared/domains/raw/models';
import type { Prisma, Raw as PrismaRaw } from '@prisma/client';
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

	public toCreateInput(domain: RawIngredientCreate): Prisma.RawCreateInput {
		return {
			name: domain.name,
			container: domain.container,
			proteins: new Decimal(domain.proteins),
			carbs: new Decimal(domain.carbs),
			fat: new Decimal(domain.fat),
			creator: {
				connect: {
					id: this.session?.user?.userId
				}
			}
		};
	}

	public toUpdateInput(domain: RawIngredientUpdate): Prisma.RawUpdateInput {
		return {
			name: domain.name,
			container: domain.container,
			proteins: new Decimal(domain.proteins),
			carbs: new Decimal(domain.carbs),
			fat: new Decimal(domain.fat),
			creator: {
				connect: {
					id: this.session?.user?.userId
				}
			}
		};
	}
}
