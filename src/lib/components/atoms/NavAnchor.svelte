<script lang="ts">
	import { AppRailAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	export let href: string;
	export let title: string | undefined = undefined;

	let pathname = "";
	$: {
		pathname = $page?.url?.pathname ?? "";
	}
</script>

<AppRailAnchor selected={pathname === href} {href} {title}>
	<slot />
</AppRailAnchor>
