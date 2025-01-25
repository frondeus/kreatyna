import type { Prisma, Tag, TagCategory } from "@prisma/client";
import type { z } from "zod";
import type { TagConnectSchema } from "./schema";
import type { tagWithCategory, tagWithCategoryName } from "./queries";

type TagWithCategoryName = Prisma.TagGetPayload<typeof tagWithCategoryName>;
type TagWithCategory = Prisma.TagGetPayload<typeof tagWithCategory>;

interface TagSearch {
  name: string;
  category: string;
}

interface TagCreateOne {
  name: string;
  category: string;
}

interface TagSelect {
  id: number;
  name: string;
  category: string;
}

type TagConnect = z.infer<typeof TagConnectSchema>;

export type {
  Tag,
  TagCategory,
  TagWithCategory,
  TagWithCategoryName,
  TagCreateOne,
  TagConnect,
  TagSelect,
  TagSearch,
};
