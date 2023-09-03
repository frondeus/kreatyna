<script lang="ts">
	import type { ShoppingListState } from '@prisma/client';
	import ShoppingListCheckbox, { State } from './ShoppingListCheckbox.svelte';

	export let id: number;
	export let name: string;
	export let amount: number;
	export let unit: string | null;
	export let state: ShoppingListState;

	$: isTODO = state === 'TODO';
	let checkboxState: State = State.Default;
	$: {
		switch (state) {
			case 'TODO':
				checkboxState = State.Default;
				break;
			case 'IN_CART':
				checkboxState = State.Checked;
				break;
			case 'BOUGHT':
				checkboxState = State.Archived;
				break;
			case 'NOT_BUING':
				checkboxState = State.Crossed;
		}
	}
</script>

<li
	class="text-surface-100 p-6 flex items-center space-x-4 justify-between"
	class:bg-surface-800={!isTODO}
	class:bg-surface-700={isTODO}
	data-id={id}
>
	<span class="basis-1/10">
		<ShoppingListCheckbox state={checkboxState} />
	</span>
	<span class="text-left grow"> {name} </span>
	{#if amount != 1 || unit}
		<span class="basis-1/10">
			{amount}{#if unit}{unit} {/if}
		</span>
	{/if}
</li>
