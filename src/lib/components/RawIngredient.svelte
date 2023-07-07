<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { RawIngredient } from '$lib/types';
	import IconDelete from '~icons/mdi/delete';

	export let raw: RawIngredient;

	let { name, container, protein, carbs, fat } = raw;

	const dispatch = createEventDispatcher();

	const remove = () => {
		dispatch('remove');
	};
</script>

<tr>
	<td>
		<input class="input" type="text" bind:value={$name} />
	</td>
	<td>{$container}g</td>
	<td>
		<input class="input" type="number" bind:value={$protein} />
	</td>
	<td>
		<input class="input" type="number" bind:value={$carbs} />
	</td>
	<td>
		<input class="input" type="number" bind:value={$fat} />
	</td>
	<td>
		<button type="button" class="btn-icon variant-soft" on:click={remove}>
			<IconDelete />
		</button>
	</td>
</tr>
