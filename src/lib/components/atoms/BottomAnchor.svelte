<script lang="ts">
	import { getContext } from '$shared/context';
	import { TabAnchor } from '@skeletonlabs/skeleton';

	export let href: string;

	const page = getContext().page;

	let pathname = $page.url.pathname ?? '';
</script>

<TabAnchor {href} selected={pathname === href}>
	<svelte:fragment slot="lead">(ic)</svelte:fragment>
	<slot />
</TabAnchor>
