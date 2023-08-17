<script lang="ts">
	import { TabAnchor } from '@skeletonlabs/skeleton';
	import { getContext } from 'svelte';

	export let href: string;

	let pathname = getContext<URL>('url')?.pathname ?? "";
</script>

<TabAnchor {href} selected={pathname === href}>
	<svelte:fragment slot="lead">(ic)</svelte:fragment>
	<slot />
</TabAnchor>
