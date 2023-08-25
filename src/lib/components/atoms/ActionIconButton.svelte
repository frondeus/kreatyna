<script lang="ts">
	import { getContext } from '$shared/context';

	export let href: string | undefined = undefined;

	const page = getContext().page;
	$: active = href && $page.url.pathname.startsWith(href);
</script>

{#if href}
	<a class="btn-icon" class:variant-soft={!active} class:variant-filled-primary={active} {href}>
		<slot />
	</a>
{:else}
	<span class="btn-icon variant-soft" on:click on:keypress role="button" tabindex="0">
		<slot />
	</span>
{/if}
