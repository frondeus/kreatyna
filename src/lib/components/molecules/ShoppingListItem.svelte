<script lang="ts">
	import type { ShoppingListState } from '@prisma/client';
	import ShoppingListCheckbox, { State } from '$components/atoms/ShoppingListCheckbox.svelte';
	import { createEventDispatcher } from 'svelte';
	import type { IngredientWithRecipe } from '$shared/domains/recipe/models';
	import IconArrowUp from '~icons/mdi/menu-up';
	import { slide } from 'svelte/transition';

	export let id: string;
	export let name: string;
	export let amount: number;
	export let unit: string | null;
	export let state: ShoppingListState | 'PROCESSING';
	export let ingredients: IngredientWithRecipe[];

	const dispatch = createEventDispatcher();
	let checkboxState: State = State.Default;
	let bgColor = 'bg-surface-700';
	let extended = false;
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

	const onCheck = (event: Event) => {
		event.stopPropagation();
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
		extended = false;
		if (amount <= 0) {
			amount = 1;
		}
		dispatch('stateChange', { state: newState, id, amount });
		// state = newState;
	};

	const toggleExtended = () => {
		extended = !extended;
	};
</script>

<li>
	<div
		class="text-surface-100 p-6 flex items-center space-x-4 justify-between {bgColor}"
		on:click={toggleExtended}
		on:keypress={toggleExtended}
		role="button"
		tabindex="0"
	>
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
	</div>
</li>

{#if extended}
	<li class="text-surface-100 p-2 bg-surface-800" transition:slide>
		<table class="table-compact w-full text-s">
			<thead class="border-b border-surface-700">
				<tr>
					<th>Recipe</th>
					<th>Amount</th>
				</tr>
			</thead>
			<tbody class="w-full divide-y divide-surface-700">
				{#each ingredients as ingredient}
					<tr class="w-full">
						<td>
							<span class="truncate whitespace-break-spaces">
								{ingredient.recipe.name}
							</span>
						</td>
						<td>{ingredient.amount} {ingredient.unit}</td>
					</tr>
				{/each}
			</tbody>
		</table>
		<div class="flex justify-center">
			<button on:click={toggleExtended} on:keypress={toggleExtended} title="Hide">
				<IconArrowUp />
			</button>
		</div>
	</li>
{/if}
