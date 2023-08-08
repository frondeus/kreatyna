import type { PrismaClient } from '@prisma/client';
import type { RawMap } from './map';
import type {
	RawIngredient,
	RawIngredientCreate,
	RawIngredientUpdate
} from '$lib/shared/domains/raw/models';

export class RawService {
	map: RawMap;
	prisma: PrismaClient;

	constructor(map: RawMap, prisma: PrismaClient) {
		this.map = map;
		this.prisma = prisma;
	}

	async getAll(): Promise<RawIngredient[]> {
		const rawIngredients = await this.prisma.raw.findMany({
			select: {
				id: true,
				name: true,
				container: true,
				proteins: true,
				carbs: true,
				fat: true,
				created_by: true
			}
		});

		return rawIngredients?.map((r) => this.map.toDomain(r));
	}

	async getOne(id: number): Promise<RawIngredient | null> {
		const data = await this.prisma.raw.findUnique({
			where: { id }
		});

		if (data === null) return null;

		return this.map.toDomain(data);
	}

	async update(raw: RawIngredientUpdate): Promise<void> {
		await this.prisma.raw.update({
			where: { id: raw.id },
			data: this.map.toUpdateInput(raw)
		});
	}

	async create(raw: RawIngredientCreate): Promise<void> {
		await this.prisma.raw.create({
			data: this.map.toCreateInput(raw)
		});
	}

	async remove(id: number): Promise<void> {
		await this.prisma.raw.delete({
			where: { id }
		});
	}
}
