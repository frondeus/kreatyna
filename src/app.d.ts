import type { PrismaClient } from '@prisma/client';
import type * as Icons from 'unplugin-icons/types/svelte';
import type { RawService } from '$server/domains/raw/service';
import type { RecipeService } from '$server/domains/recipe/service';
import type { AIService } from '$server/domains/ai/service';
import type { TagService } from '$server/domains/tag/service';

declare global {
	namespace Lucia {
		type Auth = import('$lib/server/lucia').Auth;
		type DatabaseUserAttributes = {
			username: string;
			is_admin: boolean;
			full_name: string;
		};
		// type DatabaseSessionAttributes = {};
	}
	namespace App {
		interface Locals {
			prisma: PrismaClient;
			session: Session | null;
			setSession: (session: Session | null) => void;
			services: {
				raw: RawService;
				recipe: RecipeService;
				ai: AIService;
				tag: TagService;
			};
		}
		interface PageData {
			session: Session | null;
		}

		// interface Error {}
		// interface Platform {}
	}
}

/// <reference types="@sveltejs/kit" />
/// <reference types="unplugin-icons/types/svelte" />

export default Icons;
export {};
