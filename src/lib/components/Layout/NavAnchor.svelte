<script lang="ts">
	import { AppRailAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	export let href: string;
	export let title: string | undefined = undefined;
</script>

<AppRailAnchor selected={$page.url.pathname === href} {href} {title}>
	<slot />
</AppRailAnchor>
