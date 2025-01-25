<script lang="ts">
	import type { ShoppingListCategoryForItem } from '$shared/domains/shopping/models';

	export let name: string;
	export let categories: ShoppingListCategoryForItem[];
	export let category: ShoppingListCategoryForItem | null;

	export let onSubmit: (category: ShoppingListCategoryForItem) => void;

	$: category ? onSubmit(category) : null;

	const select = (cat: ShoppingListCategoryForItem) => {
		category = cat;
	};
</script>

<div class="space-y-4">
	<h2 class="h2">Select category for {name}</h2>
	{#if category}
		<h3 class="h3">Selected: {category.name}</h3>
	{/if}

	<ul class="flex flex-wrap w-full text-2xl justify-around">
		{#each categories as cat (cat.id)}
			<li class:bg-surface-600={cat.id === category?.id}>
				<button title={cat.name} on:click={() => select(cat)} on:keypress={() => select(cat)}>
					{#if cat.icon.trim().length === 0}
						&#x2753;
					{:else}
						{cat.icon}
					{/if}
				</button>
			</li>
		{/each}
	</ul>
</div>
