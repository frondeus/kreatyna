<script lang="ts">
	import { page } from '$app/stores';
	import { TabAnchor, TabGroup } from '@skeletonlabs/skeleton';

	interface Anchor {
		href: string;
		title: string;
	}

	const anchors: Anchor[] = [
		{ href: '/browse/recipe', title: 'Recipes' },
		{ href: '/browse/raw', title: 'Raw ingredients' }
	];
</script>

<TabGroup>
	{#each anchors as { href, title } (href)}
		<TabAnchor {href} selected={$page.url.pathname.startsWith(href)}>{title}</TabAnchor>
	{/each}

	<svelte:fragment slot="panel">
		<slot />
	</svelte:fragment>
</TabGroup>
