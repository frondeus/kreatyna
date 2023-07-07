<script>
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';

	import { AppShell } from '@skeletonlabs/skeleton';

	import { invalidate } from '$app/navigation';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/Layout/Sidebar.svelte';

	export let data;

	let { supabase, session } = data;
	$: ({ supabase, session } = data);

	onMount(() => {
		const { data } = supabase.auth.onAuthStateChange((_event, _session) => {
			if (_session?.expires_at !== session?.expires_at) {
				invalidate('supabase:auth');
			}
		});

		return () => data.subscription.unsubscribe();
	});
</script>

<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar isLoggedIn={!!session} />
	</svelte:fragment>
	<div class="container mx-auto p-8 space-y-8">
		<slot />
	</div>
</AppShell>
