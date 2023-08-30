<script lang="ts">
	import { getContext } from '$shared/context';

	export let href: string | undefined = undefined;
	export let title: string | undefined = undefined;

	const page = getContext().page;
	$: active = href && $page.url.pathname.startsWith(href);
</script>

{#if href}
	<a
		class="btn-icon xl:btn-icon-sm"
		class:variant-soft={!active}
		class:variant-filled-primary={active}
		{title}
		{href}
	>
		<slot />
	</a>
{:else}
	<span
		class="btn-icon variant-soft xl:btn-icon-sm"
		on:click
		on:keypress
		role="button"
		tabindex="0"
		{title}
	>
		<slot />
	</span>
{/if}
