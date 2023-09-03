<script lang="ts">
	import ShoppingList from '$components/pages/ShoppingList.svelte';
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { items }: { items: ShoppingListItem[] } = data;
	$: ({ items } = data);
</script>

<ShoppingList {items} />
