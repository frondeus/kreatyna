<script lang="ts">
	import IconPencil from '~icons/mdi/pencil';
	import ActionIconButton from './buttons/ActionIconButton.svelte';

	export let amount: number;
	export let unit: string | undefined = undefined;
	export let editable: boolean = false;

	$: showAmount = amount > 1 || unit !== undefined;
</script>

{#if editable}
	{#if showAmount}
		<span
			class="btn xl:btn-sm variant-soft-surface max-lg:px-4"
			role="button"
			tabindex="0"
			title="Edit amount or unit"
			on:click
			on:keypress
		>
			{amount}
			{#if unit !== undefined}
				{unit}
			{/if}
		</span>
	{:else}
		<ActionIconButton on:click on:keypress>
			<IconPencil />
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
