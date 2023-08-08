import type { PrismaClient } from '@prisma/client';
import type { RawService } from '$lib/server/domains/raw/service';
import type { RecipeService } from '$lib/server/domains/recipe/service';
import type { AIService } from '$lib/server/domains/ai/service';

declare global {
	namespace Lucia {
		type Auth = import('$lib/server/lucia').Auth;
		type DatabaseUserAttributes = {
			username: string;
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
				recipy: RecipeService;
				ai: AIService;
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

import type * as Icons from 'unplugin-icons/types/svelte';
import type { RecipeService } from '$lib/server/domains/recipe/service';
import type { AIService } from '$lib/server/domains/ai/service';
export default Icons;
export {};
