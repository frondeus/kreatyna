<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import ShoppingListItemComponent from '$components/atoms/ShoppingListItem.svelte';

	export let items: ShoppingListItem[];
</script>

<ul class="divide-y divide-surface-600">
	{#each items as { id, name, amount, unit, state } (id)}
		<ShoppingListItemComponent {id} {name} {amount} {unit} {state} />
	{/each}
</ul>
