<script lang="ts">
	import type { Proposal } from '$components/molecules/ShoppingListItemProposal.svelte';
	import AddShoppingListItem from '$components/pages/AddShoppingListItem.svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import type { ShoppingListItem } from '$shared/domains/shopping/models';

	export let data: PageData;
	let { items } = data;
	$: ({ items } = data);

	let proposals: Proposal[] = [];

	const mapItemToProposal: (item: ShoppingListItem) => Proposal = ({
		id,
		name,
		state,
		unit,
		amount
	}: ShoppingListItem) => {
		console.log({ name, unit, amount, state });
		return {
			id,
			name,
			amount,
			unit: unit ?? undefined,
			state
		};
	};

	$: {
		proposals = items.map(mapItemToProposal);
		console.log({ proposals });
	}

	const onSubmit = async (event: CustomEvent<Proposal[]>) => {
		// console.log('onSubmit', event.detail);
		const res = await fetch('/shop/upsert', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(
				event.detail.map((p) => ({
					...p,
					state: p.amount === 0 && p.state !== 'BOUGHT' ? 'NOT_BUING' : p.state
				}))
			)
		});
		if (res.status !== 200) {
			console.error('Error updating item');
			return;
		}

		goto('/shop');
	};
</script>

<AddShoppingListItem {proposals} on:submit={onSubmit} />
