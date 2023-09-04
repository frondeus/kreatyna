<script lang="ts">
	import ShoppingList from '$components/pages/ShoppingList.svelte';
	import type { ShoppingListItem, ShoppingListState } from '$shared/domains/shopping/models';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invalidateAll } from '$app/navigation';

	export let data: PageData;
	let { items }: { items: ShoppingListItem[] } = data;
	$: ({ items } = data);

	const onStateChange = async (event: CustomEvent<{ state: ShoppingListState; id: number }>) => {
		const res = await fetch('/shop/update', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(event.detail)
		});
		if (res.status !== 200) {
			console.error('Error updating item');
			return;
		}
	};

	onMount(() => {
		const interval = setInterval(() => {
			invalidateAll();
		}, 500);

		return () => {
			clearInterval(interval);
		};
	});
</script>

<ShoppingList {items} on:stateChange={onStateChange} />
