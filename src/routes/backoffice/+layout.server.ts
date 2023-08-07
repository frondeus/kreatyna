import { error } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals: { auth, prisma } }) => {
	const session = await auth.validate();
	const userId = session?.user?.userId;

	if (!userId) {
		throw error(403, 'Unauthorized');
	}

	const isAdmin = await prisma.user.findUnique({
		where: { id: userId },
		select: { is_admin: true }
	});

	if (!isAdmin) {
		throw error(403, 'Unauthorized');
	}

	return {};
};
