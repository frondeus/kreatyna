<script lang="ts">
	import IconScale from '~icons/mdi/scale-balance';
	import ActionIconButton from './buttons/ActionIconButton.svelte';

	export let amount: number;
	export let unit: string | undefined = undefined;
	export let editable: boolean = false;

	$: showAmount = amount > 1 || unit !== undefined;
	const title = 'Edit amount or unit';
</script>

{#if editable}
	{#if showAmount}
		<span
			class="btn xl:btn-sm variant-soft-surface max-lg:px-4"
			role="button"
			tabindex="0"
			{title}
			on:click
			on:keypress
		>
			{amount}
			{#if unit !== undefined}
				{unit}
			{/if}
		</span>
	{:else}
		<ActionIconButton {title} on:click on:keypress>
			<IconScale />
		</ActionIconButton>
	{/if}
{:else}
	<span>
		{#if showAmount}
			{amount}
			{#if unit !== undefined}
				{unit}
			{/if}
		{/if}
	</span>
{/if}
