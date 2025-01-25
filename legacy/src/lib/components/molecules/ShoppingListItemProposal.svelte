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
	import ShoppingListProposalUnit from '$components/atoms/ShoppingListProposalUnit.svelte';
	import EditShoppingListProposalUnit from '$components/organisms/EditShoppingListProposalUnit.svelte';
	import { drawerStore, popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { v4 } from 'uuid';

	// UUID
	export let id: string;
	export let name: string;
	export let unit: string | undefined = undefined;
	export let state: ShoppingListState;
	export let amount = 0;

	let checkboxState: CheckboxState = CheckboxState.Default;

	$: {
		if (amount > 0 && state !== 'BOUGHT') {
			checkboxState = CheckboxState.Added;
		} else {
			checkboxState = CheckboxState.Default;
		}
	}

	const popupId = v4();
	const namePopup: PopupSettings = {
		event: 'click',
		target: popupId,
		placement: 'bottom'
	};

	const dispatch = createEventDispatcher();

	const onIncrease = () => {
		if (amount <= 0) {
			amount = 0;
			dispatch('add', { name, amount: amount + 1, id, unit, state });
		}
		if (state !== 'BOUGHT') {
			amount += 1;
		}
		state = 'TODO';
	};
	const onDecrease = () => {
		if (amount > 0) {
			amount -= 1;
		}
		if (amount <= 0) {
			state = 'NOT_BUING';
		}
	};
	const onEditUnit = () => {
		drawerStore.open({
			position: 'bottom',
			height: '12',
			meta: {
				component: EditShoppingListProposalUnit,
				props: {
					name,
					unit,
					amount,
					onSubmit: (a: number, u: string | undefined) => {
						amount = a;
						unit = u;
					}
				}
			}
		});
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
	<span class="text-left grow truncate" use:popup={namePopup}>
		{name}
	</span>
	<div class="card p-2 variant-filled-surface z-10" data-popup={popupId}>
		{name}
	</div>
	<span class="basis-3/10 flex gap-3 items-center">
		<ShoppingListProposalUnit
			bind:amount
			bind:unit
			editable={amount > 0 && state !== 'BOUGHT'}
			on:click={onEditUnit}
			on:keypress={onEditUnit}
		/>

		<span class="text-error-500">
			{#if state !== 'BOUGHT'}
				{#if amount == 1}
					<ActionIconButton on:click={onDecrease} on:keypress={onDecrease}>
						<IconCross class="text-error-500" />
					</ActionIconButton>
				{:else if amount > 1}
					<ActionIconButton on:click={onDecrease} on:keypress={onDecrease}>
						<IconMinus class="text-error-500" />
					</ActionIconButton>
				{/if}
			{/if}
		</span>
	</span>
</li>
