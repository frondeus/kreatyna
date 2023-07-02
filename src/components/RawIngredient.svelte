<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { RawIngredient } from '../types';

	export let raw: RawIngredient;

	let { name, container, protein, carbs, fat } = raw;

	const dispatch = createEventDispatcher();

	const remove = () => {
		dispatch('remove');
	};
</script>

<tr>
	<th>
		<input class="input" type="text" bind:value={$name} />
	</th>
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
		<button type="button" class="btn-icon variant-filled" on:click={remove}>remove</button>
	</td>
</tr>
