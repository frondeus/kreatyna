import type { PrismaClient } from '@prisma/client';
import type { AuthRequest } from 'lucia';

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
			auth: AuthRequest<Lucia.Auth>;
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
export default Icons;
export {};
