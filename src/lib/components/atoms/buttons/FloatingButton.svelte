<script lang="ts">
	export let href: string | undefined = undefined;
	export let title: string | undefined = undefined;

	const klass = 'xl:hidden btn-icon btn-icon-xl variant-filled-primary fixed bottom-6 right-6';
</script>

{#if href}
	<a {href} class={klass} {title}><slot /></a>
{:else}
	<span class={klass} on:click on:keypress role="button" tabindex="0" {title}>
		<slot />
	</span>
{/if}
