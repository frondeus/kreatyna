<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import ShoppingListItemComponent from '$components/molecules/ShoppingListItem.svelte';
	import type { ShoppingListState } from '@prisma/client';
	import { sortByState } from '$shared/domains/shopping/utils';

	export let items: ShoppingListItem[];

	const onStateChange = (event: CustomEvent<{ state: ShoppingListState; id: number }>) => {
		console.log('Changed!', event.detail);
	};

	$: sortedItems = items.sort(sortByState);
</script>

<ul class="divide-y divide-surface-600">
	{#each sortedItems as { id, name, amount, unit, state } (id)}
		<ShoppingListItemComponent
			{id}
			{name}
			{amount}
			{unit}
			bind:state
			on:stateChange={onStateChange}
		/>
	{/each}
</ul>
