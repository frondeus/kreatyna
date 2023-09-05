<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import ShoppingListItemComponent from '$components/molecules/ShoppingListItem.svelte';
	import { sortByState } from '$shared/domains/shopping/utils';
	import FloatingButton from '$components/atoms/buttons/FloatingButton.svelte';
	import IconPlus from '~icons/mdi/plus';
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import IconBasket from '~icons/mdi/basket-unfill';
	import { createEventDispatcher } from 'svelte';

	export let items: ShoppingListItem[];

	const dispatch = createEventDispatcher();

	$: sortedItems = items.filter((i) => i.state !== 'BOUGHT').sort(sortByState);
	$: atLeastOneInCart = sortedItems.some((i) => i.state === 'IN_CART');

	const onFinish = () => {
		dispatch('finish', sortedItems);
	};
</script>

<div>
	<header class="max-lg:hidden bg-surface-800 p-2 gap-2 flex justify-end">
		{#if sortedItems.length > 0 && atLeastOneInCart}
			<ActionIconButton title="Finish" on:click={onFinish} on:keypress={onFinish}>
				<IconBasket class="text-success-500" />
			</ActionIconButton>
		{/if}
		<ActionIconButton title="Add" href="/shop/add">
			<IconPlus />
		</ActionIconButton>
	</header>

	<ul class="divide-y divide-surface-600">
		{#each sortedItems as { id, name, amount, unit, state } (id)}
			<ShoppingListItemComponent {id} {name} {amount} {unit} {state} on:stateChange />
		{/each}
	</ul>

	<div class="h-20">
		<!-- <button class="variant-filled-secondary"> Finish </button> -->
	</div>
</div>

<FloatingButton href="/shop/add">
	<IconPlus />
</FloatingButton>
