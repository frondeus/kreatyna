<script lang="ts">
	import type { ShoppingListState } from '@prisma/client';
	import ShoppingListCheckbox, { State } from '$components/atoms/ShoppingListCheckbox.svelte';
	import { createEventDispatcher } from 'svelte';
	import type { IngredientWithRecipe } from '$shared/domains/recipe/models';

	export let id: string;
	export let name: string;
	export let amount: number;
	export let unit: string | null;
	export let state: ShoppingListState | 'PROCESSING';
	export let ingredients: IngredientWithRecipe[];

	$: console.log({ ingredients });

	const dispatch = createEventDispatcher();
	let checkboxState: State = State.Default;
	let bgColor = 'bg-surface-700';
	$: {
		switch (state) {
			case 'TODO':
				checkboxState = State.Default;
				bgColor = 'bg-surface-700';
				break;
			case 'IN_CART':
				checkboxState = State.Checked;
				bgColor = 'bg-surface-800';
				break;
			case 'BOUGHT':
				checkboxState = State.Archived;
				bgColor = 'bg-slate-600';
				break;
			case 'NOT_BUING':
				checkboxState = State.Crossed;
				bgColor = 'bg-surface-800';
				break;
			case 'PROCESSING':
				checkboxState = State.Processing;
				break;
		}
	}

	const onCheck = () => {
		let newState: ShoppingListState | 'PROCESSING';
		switch (state) {
			case 'TODO':
				newState = 'IN_CART';
				break;
			case 'IN_CART':
				newState = 'NOT_BUING';
				break;
			case 'BOUGHT':
				newState = 'TODO';
				break;
			case 'NOT_BUING':
				newState = 'TODO';
				break;
			case 'PROCESSING':
				return;
		}
		state = 'PROCESSING';
		if (amount <= 0) {
			amount = 1;
		}
		dispatch('stateChange', { state: newState, id, amount });
		// state = newState;
	};
</script>

<li class="text-surface-100 p-6 flex items-center space-x-4 justify-between {bgColor}">
	<span class="basis-1/10">
		<ShoppingListCheckbox state={checkboxState} on:click={onCheck} on:keypress={onCheck} />
	</span>
	<span class="text-left grow" class:line-through={state === 'NOT_BUING'}>
		{name}
	</span>
	{#if amount != 1 || unit}
		<span class="basis-1/10 whitespace-nowrap">
			{amount}
			{#if unit} {unit} {/if}
		</span>
	{/if}
</li>
