<script lang="ts">
	import ShoppingList from '$components/pages/ShoppingList.svelte';
	import type { ShoppingListItem, ShoppingListState } from '$shared/domains/shopping/models';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';

	export let data: PageData;
	let { items }: { items: ShoppingListItem[] } = data;
	$: ({ items } = data);

	// $: console.log('DATA:', data);
	// $: console.log('ITEMS:', items[0].state);

	onMount(() => {
		const interval = setInterval(async () => {
			await invalidate('data:shopping-items');
			// console.log('invalidated');
		}, 1000);

		return () => {
			clearInterval(interval);
		};
	});

	const onFinish = async (event: CustomEvent<ShoppingListItem[]>) => {
		const items = event.detail.map((item) => ({
			...item,
			// amount: 0,
			// unit: undefined,
			state: 'BOUGHT'
		}));

		await invalidate('data:shopping-items');
		const res = await fetch('/shop/update', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(items)
		});
		if (res.status !== 200) {
			console.error('Error updating item');
			return;
		}
	};

	const onStateChange = async (
		event: CustomEvent<{
			state: ShoppingListState;
			id: string;
			amount: number;
			unit: string | undefined;
		}>
	) => {
		// console.log('onStateChange', event.detail);
		await invalidate('data:shopping-items');
		const res = await fetch('/shop/update', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify([event.detail])
		});
		if (res.status !== 200) {
			console.error('Error updating item');
			return;
		}
		// items = items;
	};
</script>

<ShoppingList {items} on:stateChange={onStateChange} on:finish={onFinish} />
