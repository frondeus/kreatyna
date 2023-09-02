import { prisma } from '@lucia-auth/adapter-prisma';
import { PrismaClient } from '@prisma/client';
import { Env, lucia } from 'lucia';
import * as fs from 'fs';

const db = new PrismaClient();

async function main() {
	const env: Env = process.argv[2] === 'PROD' ? 'PROD' : 'DEV';
	const auth = lucia({
		env,
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
			username: 'frondeus',
			is_admin: true,
			full_name: 'Wojciech Polak'
		}
	});

	await auth.createUser({
		key: {
			providerId: 'username',
			providerUserId: 'klaudia',
			password: 'kreatyna123'
		},
		attributes: {
			username: 'klaudia',
			is_admin: false,
			full_name: 'Klaudia Polak'
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
