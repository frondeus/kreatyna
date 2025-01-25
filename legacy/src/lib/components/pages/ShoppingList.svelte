<script lang="ts">
	import type {
		ShoppingListCategoryForItem,
		ShoppingListItem
	} from '$shared/domains/shopping/models';
	import ShoppingListItemComponent from '$components/molecules/ShoppingListItem.svelte';
	import { sortByState } from '$shared/domains/shopping/utils';
	import FloatingButton from '$components/atoms/buttons/FloatingButton.svelte';
	import IconPlus from '~icons/mdi/plus';
	import IconNuke from '~icons/mdi/nuke';
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import IconBasket from '~icons/mdi/basket-unfill';
	import { createEventDispatcher } from 'svelte';

	export let items: ShoppingListItem[];
	export let categories: ShoppingListCategoryForItem[];

	const dispatch = createEventDispatcher();

	$: sortedItems = items.filter((i) => i.state !== 'BOUGHT').sort(sortByState);
	$: atLeastOneInCart = sortedItems.some((i) => i.state === 'IN_CART');

	const onFinish = () => {
		dispatch('finish', sortedItems);
	};

	const onClear = () => {
		dispatch('clear', sortedItems);
	};
</script>

<div>
	<header class="max-lg:hidden bg-surface-800 p-2 gap-2 flex justify-end">
		{#if sortedItems.length > 0}
			<ActionIconButton
				title="Clear all (inc. not bought)"
				on:click={onClear}
				on:keypress={onClear}
			>
				<IconNuke class="text-error-500" />
			</ActionIconButton>
		{/if}
		{#if sortedItems.length > 0 && atLeastOneInCart}
			<ActionIconButton title="Finish shopping" on:click={onFinish} on:keypress={onFinish}>
				<IconBasket class="text-success-500" />
			</ActionIconButton>
		{/if}
		<ActionIconButton title="Add" href="/shop/add">
			<IconPlus />
		</ActionIconButton>
	</header>

	<ul class="divide-y divide-surface-600">
		{#each sortedItems as { id, name, amount, unit, state, ingredients, category } (id)}
			<ShoppingListItemComponent
				{id}
				{name}
				{amount}
				{unit}
				{state}
				{ingredients}
				{category}
				{categories}
				on:update
			/>
		{/each}
	</ul>

	<div class="lg:hidden flex justify-stretch my-2 gap-2">
		{#if sortedItems.length}
			<button
				on:click={onClear}
				on:keypress={onClear}
				class="btn btn-md variant-ringed-error flex-1 rounded-none"
				title="Including not bought">Clear all</button
			>
		{/if}
		{#if sortedItems.length > 0 && atLeastOneInCart}
			<button
				on:click={onFinish}
				on:keypress={onFinish}
				class="btn btn-xl variant-ghost-success flex-1 rounded-none">Finish shopping</button
			>
		{/if}
	</div>
	<div class="h-10" />
	<div class="h-20" />
</div>

<FloatingButton href="/shop/add">
	<IconPlus />
</FloatingButton>
