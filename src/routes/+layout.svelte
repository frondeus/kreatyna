<script>
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';

	import { AppShell, Modal, Toast } from '@skeletonlabs/skeleton';

	import { invalidate } from '$app/navigation';
	import { onMount } from 'svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import Sidebar from '$lib/components/Layout/Sidebar.svelte';
	import BottomBar from '$lib/components/Layout/BottomBar.svelte';

	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

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

<Modal />

<AppShell slotSidebarLeft="w-0 lg:w-64" slotFooter="lg:hidden">
	<svelte:fragment slot="sidebarLeft">
		<Sidebar isLoggedIn={!!session} />
	</svelte:fragment>
	<div class="lg:mx-auto lg:p-8 lg:space-y-8">
		<slot />
	</div>
	<svelte:fragment slot="footer">
		<BottomBar />
	</svelte:fragment>
</AppShell>

<Toast />
