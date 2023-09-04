<script context="module" lang="ts">
	export interface Proposal {
		name: string;
		amount: number;
		id: string;
		unit?: string;
		state: ShoppingListState;
	}
</script>

<script lang="ts">
	import ShoppingListProposalCheckbox, {
		State as CheckboxState
	} from '$components/atoms/ShoppingListProposalCheckbox.svelte';
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import IconMinus from '~icons/mdi/minus-thick';
	import IconCross from '~icons/mdi/close-thick';
	import { createEventDispatcher } from 'svelte';
	import type { ShoppingListState } from '@prisma/client';

	// UUID
	export let id: string;
	export let name: string;
	export let unit: string | undefined = undefined;
	export let amount = 0;

	let checkboxState: CheckboxState = CheckboxState.Default;

	$: {
		if (amount > 0) {
			checkboxState = CheckboxState.Added;
		} else {
			checkboxState = CheckboxState.Default;
		}
	}

	const dispatch = createEventDispatcher();

	const onIncrease = () => {
		if (amount <= 0) {
			amount = 0;
			dispatch('add', { name, amount: amount + 1, id, unit });
		}
		amount += 1;
	};
	const onDecrease = () => {
		if (amount > 0) {
			amount -= 1;
		}
	};
</script>

<li class="flex items-center space-x-4 justify-between h-11">
	<span class="basis-1/10">
		<ShoppingListProposalCheckbox
			state={checkboxState}
			on:click={onIncrease}
			on:keypress={onIncrease}
		/>
	</span>
	<span class="text-left grow">
		{name}
	</span>
	<span class="basis-3/10 flex gap-2 items-center">
		{#if amount > 1}
			{amount}
		{/if}

		<span class="text-error-500">
			{#if amount == 1}
				<ActionIconButton on:click={onDecrease} on:keypress={onDecrease}>
					<IconCross class="text-error-500" />
				</ActionIconButton>
			{:else if amount > 1}
				<ActionIconButton on:click={onDecrease} on:keypress={onDecrease}>
					<IconMinus class="text-error-500" />
				</ActionIconButton>
			{/if}
		</span>
	</span>
</li>
