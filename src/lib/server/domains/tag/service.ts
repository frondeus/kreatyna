import type { PrismaClient } from '@prisma/client';
import type { TagCreateOne, TagSearch, TagWithCategory } from '$shared/domains/tags/models';

export class TagService {
	prisma: PrismaClient;

	constructor(prisma: PrismaClient) {
		this.prisma = prisma;
	}

	async findIds(search: TagSearch): Promise<number[]> {
		return (
			await this.prisma.tag.findMany({
				select: {
					id: true
				},
				where: {
					name: search.name,
					category: {
						name: search.category
					}
				}
			})
		).map((t) => t.id);
	}

	async find(search: TagSearch): Promise<TagWithCategory[]> {
		return await this.prisma.tag.findMany({
			include: {
				category: true
			},
			where: {
				name: search.name,
				category: {
					name: search.category
				}
			}
		});
	}

	async getAll(): Promise<TagWithCategory[]> {
		return await this.prisma.tag.findMany({
			include: {
				category: true
			}
		});
	}

	async create(tag: TagCreateOne) {
		await this.prisma.tag.create({
			data: {
				name: tag.name,
				category: {
					connectOrCreate: {
						where: { name: tag.category },
						create: { name: tag.category }
					}
				}
			}
		});
	}

	async remove(id: number) {
		await this.prisma.tag.delete({
			where: { id }
		});
	}
}
