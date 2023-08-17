<script lang="ts">
	export let href: string | undefined = undefined;
</script>

{#if href}
	<a class="btn-icon variant-soft" {href}>
		<slot />
	</a>
{:else}
	<span
	    class="btn-icon variant-soft"
	    on:click
	    on:keypress
	    role="button"
	    tabindex="0">
	    <slot />
	</span>
{/if}
