<script lang="ts">
	import { AppRail } from '@skeletonlabs/skeleton';
	import NavAnchor from '../atoms/NavAnchor.svelte';
	import { getContext } from 'svelte';

	export let isLoggedIn: boolean;

	const links = [
		{
			href: '/plan',
			title: 'Plan',
		},
		// {
		// 	href: '/shop',
		// 	title: 'Shop',
		// },
		// {
		// 	href: '/store',
		// 	title: 'Store',
		// },
		// {
		// 	href: '/cook',
		// 	title: 'Cook',
		// },
		// {
		// 	href: '/log',
		// 	title: 'Log',
		// },
		{
			href: '/browse',
			title: 'Browse',
		},
	];

	let pathname = getContext<URL>('url')?.pathname ?? "";
	$: classesActive = (href: string) => (href === pathname ? '!bg-primary-500' : '');
</script>

<nav class="list-nav p-4">
	<ul>
		{#each links as { href, title }}
			<li>
				<a 	class="{classesActive(href)}"
					{href}
					{title}
				>{title}</a>
			</li>
		{/each}
	</ul>
</nav>
<!-- <AppRail>
	<svelte:fragment slot="lead">
		<NavAnchor href="/">
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
				Account
			</NavAnchor>
		{:else}
			<NavAnchor href="/login" title="Login">
				Login
			</NavAnchor>
		{/if}
	</svelte:fragment>
</AppRail> -->
