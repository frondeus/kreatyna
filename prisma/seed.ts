import { prisma } from '@lucia-auth/adapter-prisma';
import { PrismaClient } from '@prisma/client';
import { lucia } from 'lucia';
import * as fs from 'fs';

const db = new PrismaClient();

async function main() {
	const auth = lucia({
		env: 'DEV',
		adapter: prisma(db, {
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

	const wojtek = await auth.createUser({
		key: {
			providerId: 'username',
			providerUserId: 'frondeus',
			password: 'kreatyna123'
		},
		attributes: {
			username: 'frondeus'
		}
	});

	await auth.createUser({
		key: {
			providerId: 'username',
			providerUserId: 'klaudia',
			password: 'kreatyna123'
		},
		attributes: {
			username: 'klaudia'
		}
	});

	await db.user.update({
		where: { id: wojtek.userId },
		data: {
			is_admin: true
		}
	});

	const seedList = fs.readdirSync('./prisma/seed');
	for (const seedFilename of seedList) {
		const seedPath = './prisma/seed/' + seedFilename;

		console.log('Seeding: ', seedPath);
		const seedFile = fs.readFileSync(seedPath, 'utf8');

		const seedParts = seedFile.split(';');
		for (const seedPart of seedParts) {
			const templatedSeedPart = seedPart.replaceAll('{{userId}}', wojtek.userId);
			await db.$executeRawUnsafe(templatedSeedPart);
		}
	}
}

main()
	.then(async () => {
		await db.$disconnect();
	})
	.catch(async (e) => {
		console.error(e);
		await db.$disconnect();
		process.exit(1);
	});
