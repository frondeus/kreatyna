<script lang="ts">
	import { TabAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	export let href: string;
</script>

<TabAnchor href={href} selected={$page.url.pathname === href}>
	<svelte:fragment slot="lead">(ic)</svelte:fragment>
  <slot />
</TabAnchor>
