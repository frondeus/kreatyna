import { addRawSchema } from "$lib/schemas";
import { importRawIngredients } from "$lib/import";
import { fail, type Actions, redirect } from "@sveltejs/kit";

export const actions: Actions = {
    default: async ({ request, locals: { getSession, supabase } }) => {
        const session = await getSession();
        if(!session) {
            return fail(403, { error: 'Unauthorized' });
        }

        const formData = await request.formData();
        const file = formData.get('xlsx') as File;
        const buffer = Buffer.from(new Uint8Array(await file.arrayBuffer()));

        const raws = await importRawIngredients(buffer, session.user.id);
        const validateResult = await addRawSchema.array().safeParseAsync(raws);


        if (!validateResult.success) {
            return fail(400, { error: validateResult.error.format() });
        }

        const { error } = await supabase.from('raw').insert(raws);

        if(error) {
            console.error(error);
            return fail(500, { error });
        }

        throw redirect(303, '/browse/raw');
    }
}
