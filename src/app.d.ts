import type { PrismaClient } from '@prisma/client';
import RawService from '$lib/server/domains/raw/service';

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
export default Icons;
export {};
