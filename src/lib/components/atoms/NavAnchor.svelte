<script lang="ts">
	import { AppRailAnchor } from '@skeletonlabs/skeleton';
	import { getContext, onMount } from 'svelte';

	export let href: string;
	export let title: string | undefined = undefined;

	let pathname = getContext<URL>('url')?.pathname ?? "";
</script>

<AppRailAnchor selected={pathname === href} {href} {title}>
	<slot />
</AppRailAnchor>
