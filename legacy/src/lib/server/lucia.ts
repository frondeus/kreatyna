import { dev } from '$app/environment';
import { prisma } from '@lucia-auth/adapter-prisma';
import { PrismaClient } from '@prisma/client';
import { lucia } from 'lucia';
import { sveltekit } from 'lucia/middleware';

const prismaClient = new PrismaClient();
export const auth = lucia({
	env: dev ? 'DEV' : 'PROD',
	middleware: sveltekit(),
	adapter: prisma(prismaClient, {
		user: 'user',
		session: 'session',
		key: 'key'
	}),
	getUserAttributes: (data) => {
		return {
			username: data.username
		};
	}
});

export type Auth = typeof auth;

//https://lucia-auth.com/guidebook/sign-in-with-username-and-password/sveltekit
