import { Prisma, type Tag, type TagCategory } from '@prisma/client';
import type { z } from 'zod';
import type { TagConnectSchema } from './schema';

const tagWithCategory = Prisma.validator<Prisma.TagFindManyArgs>()({
	include: { category: true }
});

type TagWithCategory = Prisma.TagGetPayload<typeof tagWithCategory>;

interface TagCreateOne {
	name: string;
	category: string;
}

type TagConnect = z.infer<typeof TagConnectSchema>;

export type { Tag, TagCategory, TagWithCategory, TagCreateOne, TagConnect };
