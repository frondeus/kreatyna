<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import ShoppingListItemComponent from '$components/molecules/ShoppingListItem.svelte';
	import { sortByState } from '$shared/domains/shopping/utils';

	export let items: ShoppingListItem[];

	$: sortedItems = items.sort(sortByState);
</script>

<ul class="divide-y divide-surface-600">
	{#each sortedItems as { id, name, amount, unit, state } (id)}
		<ShoppingListItemComponent {id} {name} {amount} {unit} {state} on:stateChange />
	{/each}
</ul>
