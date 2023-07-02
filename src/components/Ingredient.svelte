<script lang="ts">
	import type { Writable, Readable } from 'svelte/store';
	import { RangeSlider } from '@skeletonlabs/skeleton';

	import type { IngredientWithData } from '../types';
	import { round2 } from '../types';

	export let ingredient: Writable<number>;
	export let withData: Readable<IngredientWithData>;

	let protein = round2($withData.protein);
	let carbs = round2($withData.carbs);
	let fat = round2($withData.fat);
</script>

<tr>
	<td>{$withData.name}</td>
	<td>
		<RangeSlider name="amount" min={0} max={1000} bind:value={$ingredient} />
		<input class="input" title={`${$ingredient}`} type="number" bind:value={$ingredient} />
	</td>
	<td>{protein}</td>
	<td>{carbs}</td>
	<td>{fat}</td>
</tr>
