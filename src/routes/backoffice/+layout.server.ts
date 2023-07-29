import { error } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals: { getSession, supabase }}) => {
    const session = await getSession();
    const userId = session?.user?.id;

    if(!userId) {
        throw error(403, 'Unauthorized');
    }

    const { data: isAdmin } = await supabase.from('profiles').select('is_admin').eq('id', userId).single();

    if(!isAdmin) {
        throw error(403, 'Unauthorized');
    }

    return {}
};