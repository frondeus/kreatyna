<script lang="ts">
	import type { ShoppingListItem } from '$shared/domains/shopping/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { items }: { items: ShoppingListItem[] } = data;
	$: ({ items } = data);
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
