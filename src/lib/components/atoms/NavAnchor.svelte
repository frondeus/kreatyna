<script lang="ts">
	import { getContext } from '$shared/context';
	import { AppRailAnchor } from '@skeletonlabs/skeleton';

	export let href: string;
	export let title: string | undefined = undefined;

	const page = getContext().page;

	let pathname = $page.url.pathname ?? '';
</script>

<AppRailAnchor selected={pathname === href} {href} {title}>
	<slot />
</AppRailAnchor>
