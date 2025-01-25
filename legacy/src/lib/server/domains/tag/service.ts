import type { PrismaClient } from "@prisma/client";
import type {
  TagCreateOne,
  TagSearch,
  TagWithCategory,
} from "$shared/domains/tags/models";
import { tagWithCategory } from "$shared/domains/tags/queries";

export class TagService {
  constructor(readonly prisma: PrismaClient) {}

  async find(search: TagSearch): Promise<TagWithCategory[]> {
    return await this.prisma.tag.findMany({
      ...tagWithCategory,
      where: {
        name: search.name,
        category: {
          name: search.category,
        },
      },
    });
  }

  async getAll(): Promise<TagWithCategory[]> {
    return await this.prisma.tag.findMany({
      ...tagWithCategory,
    });
  }

  async create(tag: TagCreateOne) {
    await this.prisma.tag.create({
      data: {
        name: tag.name,
        category: {
          connectOrCreate: {
            where: { name: tag.category },
            create: { name: tag.category },
          },
        },
      },
    });
  }

  async remove(id: number) {
    await this.prisma.tag.delete({
      where: { id },
    });
  }
}
