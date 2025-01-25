import { Prisma } from "@prisma/client";

export const tagWithCategoryName = Prisma.validator<Prisma.TagFindManyArgs>()({
  select: {
    id: true,
    name: true,
    category: {
      select: {
        name: true,
      },
    },
  },
});

export const tagWithCategory = Prisma.validator<Prisma.TagFindManyArgs>()({
  select: {
    id: true,
    name: true,
    categoryId: true,
    category: {
      select: {
        id: true,
        name: true,
      },
    },
  },
});
