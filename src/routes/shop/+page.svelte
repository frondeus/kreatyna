<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import { Prisma } from '@prisma/client';

	const sortByState: (a: ShoppingListItem, b: ShoppingListItem) => number = (a, b) => {
		if (a.state !== b.state) {
			if (a.state === 'TODO') return -1;
			if (b.state === 'TODO') return 1;

			if (a.state === 'IN_CART') return -1;
			if (b.state === 'IN_CART') return 1;
		}

		return a.order - b.order;
	};

	const items: ShoppingListItem[] = [
		// from 0..20 to array of items
		...Array.from(Array(20).keys()).map((i) => ({
			id: i,
			name: `Item ${i}`,
			amount: new Prisma.Decimal((i + 1) * 100),
			unit: 'g',
			state: i % 5 == 0 ? ('IN_CART' as const) : ('TODO' as const),
			order: i
		}))
	].sort(sortByState);
</script>

<ul class="divide-y">
	{#each items as { id, name, amount, unit, state } (id)}
		<li
			class="bg-surface-800 p-6 flex items-center space-x-4 justify-between"
			class:bg-surface-700={state === 'TODO'}
		>
			<span class="basis-1/10">
				<input class="checkbox" type="checkbox" checked={state !== 'TODO'} />
			</span>
			<span class="text-left grow"> {name} </span>
			<span class="basis-1/10">
				{amount}{#if unit}{unit} {/if}
			</span>
		</li>
	{/each}
</ul>
