import type { PrismaClient } from '@prisma/client';
import type * as Icons from 'unplugin-icons/types/svelte';
import type { DependencyContainer } from 'tsyringe';

declare global {
	interface Date {
		getWeek(): number;
	}
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
			container: DependencyContainer;
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
