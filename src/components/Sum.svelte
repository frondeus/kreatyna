<script lang="ts">
	import type { Readable } from 'svelte/store';
	import type { Sum } from '../types';
	import { round2 } from '../types';

	export let sum: Readable<Sum>;

	let { protein, carbs, fat } = $sum;

	$: {
		protein = $sum.protein;
		carbs = $sum.carbs;
		fat = $sum.fat;
	}
</script>

<tr>
	<th colspan="2">Total</th>
	<td>{round2($protein, 2)}</td>
	<td>{round2($carbs, 2)}</td>
	<td>{round2($fat, 2)}</td>
</tr>
