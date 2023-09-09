<script lang="ts">
	import ShoppingList from '$components/pages/ShoppingList.svelte';
	import type {
		ShoppingListCategoryForItem,
		ShoppingListItem,
		ShoppingListState
	} from '$shared/domains/shopping/models';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { toastStore } from '@skeletonlabs/skeleton';
	import type { Category } from '$shared/domains/shopping/schema';

	export let data: PageData;
	let {
		items,
		categories
	}: { items: ShoppingListItem[]; categories: ShoppingListCategoryForItem[] } = data;
	$: ({ items, categories } = data);

	onMount(() => {
		const interval = setInterval(async () => {
			await invalidate('data:shopping-items');
		}, 1000);

		return () => {
			clearInterval(interval);
		};
	});

	const onFinish = async (event: CustomEvent<ShoppingListItem[]>) => {
		const items = event.detail.map((item) => ({
			...item,
			state: item.state === 'IN_CART' ? 'BOUGHT' : item.state,
			unit: item.unit ?? undefined,
			category: item.category
				? {
						id: item.category.id,
						name: item.category.name,
						emoji: item.category.icon
				  }
				: undefined
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
			toastStore.trigger({
				message: 'Could not update items',
				background: 'variant-filled-error'
			});
			return;
		}
	};

	const onUpdate = async (
		event: CustomEvent<{
			state: ShoppingListState;
			id: string;
			amount: number;
			unit: string | undefined;
			category: Category | undefined;
		}>
	) => {
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
			toastStore.trigger({
				message: 'Could not update item',
				background: 'variant-filled-error'
			});
			return;
		}
	};
</script>

<svelte:head>
	<title>Kreatyna - Shopping List</title>
</svelte:head>

<ShoppingList {items} {categories} on:update={onUpdate} on:finish={onFinish} />
