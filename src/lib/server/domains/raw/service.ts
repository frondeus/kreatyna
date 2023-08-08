import type { PrismaClient } from '@prisma/client';
import type { RawMap } from './map';
import type { RawIngredient } from '$lib/types';

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

	async update(raw: RawIngredient): Promise<void> {
		await this.prisma.raw.update({
			where: { id: raw.id },
			data: this.map.toDB(raw)
		});
	}

	async create(raw: RawIngredient): Promise<void> {
		await this.prisma.raw.create({
			data: this.map.toDB(raw)
		});
	}

	async deleteOne(id: number): Promise<void> {
		await this.prisma.raw.delete({
			where: { id }
		});
	}
}
