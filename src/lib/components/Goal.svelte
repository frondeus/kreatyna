<script lang="ts">
	import type { Goal } from '$lib/types';
	export let goal: Goal;

	let { protein, carbs, fat } = goal;

	// let goals: Goal[] = [];
</script>

<tr>
	<th colspan="2">Goal</th>
	<td><input class="input" type="number" bind:value={$protein} /></td>
	<td><input class="input" type="number" bind:value={$carbs} /></td>
	<td><input class="input" type="number" bind:value={$fat} /></td>
</tr>
