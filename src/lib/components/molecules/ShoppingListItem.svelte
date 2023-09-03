<script lang="ts">
	import type { ShoppingListState } from '@prisma/client';
	import ShoppingListCheckbox, { State } from '$components/atoms/ShoppingListCheckbox.svelte';
	import { createEventDispatcher } from 'svelte';

	export let id: number;
	export let name: string;
	export let amount: number;
	export let unit: string | null;
	export let state: ShoppingListState;

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
		}
	}

	const onCheck = () => {
		switch (state) {
			case 'TODO':
				state = 'IN_CART';
				break;
			case 'IN_CART':
				state = 'NOT_BUING';
				break;
			case 'BOUGHT':
				state = 'TODO';
				break;
			case 'NOT_BUING':
				state = 'TODO';
		}
		dispatch('stateChange', { state, id });
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
		<span class="basis-1/10">
			{amount}{#if unit}{unit} {/if}
		</span>
	{/if}
</li>
