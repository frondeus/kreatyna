<script lang="ts">
	import { AppRail } from '@skeletonlabs/skeleton';
	import NavAnchor from './NavAnchor.svelte';

	export let isLoggedIn: boolean;
</script>

<AppRail>
	<svelte:fragment slot="lead">
		<NavAnchor href="/">
			<!-- <IconAccountBox style={{ fontSize: '20em', color: 'red' }} /> -->
			Kreatyna
		</NavAnchor>
	</svelte:fragment>

	<NavAnchor href="/plan" title="Plan">Plan</NavAnchor>
	<NavAnchor href="/shop" title="Shop">Shop</NavAnchor>
	<NavAnchor href="/store" title="Store">Store</NavAnchor>
	<NavAnchor href="/cook" title="Cook">Cook</NavAnchor>
	<NavAnchor href="/log" title="Log">Log</NavAnchor>
	<NavAnchor href="/browse" title="Browse">Browse</NavAnchor>

	<svelte:fragment slot="trail">
		{#if isLoggedIn}
			<NavAnchor href="/account" title="Account">
				<!-- <IconAccountBox style={{ fontSize: '20em', color: 'red' }} /> -->
				Account
			</NavAnchor>
		{:else}
			<NavAnchor href="/login" title="Login">
				<!-- <IconAccountBox style={{ fontSize: '20em', color: 'red' }} /> -->
				Login
			</NavAnchor>
		{/if}
	</svelte:fragment>
</AppRail>
