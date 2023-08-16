<script lang="ts">
	import { TabAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	export let href: string;

	let pathname = "";
	$: {
		pathname = $page?.url?.pathname ?? "";
	}

</script>

<TabAnchor {href} selected={pathname === href}>
	<svelte:fragment slot="lead">(ic)</svelte:fragment>
	<slot />
</TabAnchor>
