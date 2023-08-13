<script lang="ts">
	export let href: string | undefined;
</script>

{#if href}
	<a class="btn-icon variant-soft" {href}>
		<slot />
	</a>
{/if}
