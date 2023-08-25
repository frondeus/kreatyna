<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '$root/app.postcss';

	import Navigation from '$components/molecules/Navigation.svelte';
	import { AppShell, Drawer, Modal, Toast, drawerStore } from '@skeletonlabs/skeleton';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import PageContainer from '$components/atoms/PageContainer.svelte';
	import PageHeader from '$components/molecules/PageHeader.svelte';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	export let isLoggedIn: boolean;
	$: showSidebar = isLoggedIn ? 'xl:w-64' : '';
	const onHamburger = () => {
		drawerStore.open({});
	};
</script>

<Drawer>
	<Navigation />
</Drawer>
<Modal />
<Toast />

<AppShell slotSidebarLeft="bg-surface-500/5 w-0 {showSidebar}">
	<svelte:fragment slot="header">
		<PageHeader {isLoggedIn} on:hamburger={onHamburger} />
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<Navigation />
	</svelte:fragment>
	<PageContainer>
		<slot />
	</PageContainer>
</AppShell>
