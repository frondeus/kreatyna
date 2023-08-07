import { addRawSchema } from '$lib/schemas';
import { importRawIngredients } from '$lib/import';
import { fail, type Actions, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, locals: { auth, prisma } }) => {
		const session = await auth.validate();
		if (!session) {
			return redirect(303, '/login');
		}

		const formData = await request.formData();
		const file = formData.get('xlsx') as File;
		const buffer = Buffer.from(new Uint8Array(await file.arrayBuffer()));

		const raws = await importRawIngredients(buffer, session.user.userId);
		const validateResult = await addRawSchema.array().safeParseAsync(raws);

		if (!validateResult.success) {
			return fail(400, { error: validateResult.error.format() });
		}

		await prisma.raw.createMany({
			data: raws
		});

		throw redirect(303, '/browse/raw');
	}
};
