<script>
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';

	import { AppShell, Modal, Toast } from '@skeletonlabs/skeleton';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import Sidebar from '$lib/components/molecules/Sidebar.svelte';
	import BottomBar from '$lib/components/molecules/BottomBar.svelte';

	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	export let data;
	let session = data.session;
	$: ({ session } = data);
</script>

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

<Modal />
<Toast />
