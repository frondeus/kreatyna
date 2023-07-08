
<script lang="ts">
	import { writable } from 'svelte/store';
	import type { RawIngredentsStore as RawIngredientsStore } from '$lib/stores';
	import RawIngredientRow from '$lib/components/RawIngredient.svelte';
	import IconAdd from '~icons/mdi/add';
	import rawIngredientsJson from '$lib/data/raw.json';

	export let data;

	let rawIngredients: RawIngredientsStore = writable(
		rawIngredientsJson.map((entry) => ({
			name: writable(entry.name),
			container: writable(entry.container),
			protein: writable(entry.protein),
			carbs: writable(entry.carbs),
			fat: writable(entry.fat)
		}))
	);

	const addRaw = () => {
		rawIngredients.update((raw) => {
			return [
				...raw,
				{
					name: writable(''),
					container: writable(100),
					protein: writable(0),
					carbs: writable(0),
					fat: writable(0)
				}
			];
		});
	};

	const removeRaw = (idx: number) => {
		rawIngredients.update((raw) => {
			return [...raw.slice(0, idx), ...raw.slice(idx + 1, raw.length)];
		});
	};

</script>

<div class="md:table-container">
	<table class="table">
		<thead>
			<tr>
				<th>Name</th>
				<th>Container</th>
				<th>Proteins</th>
				<th>Carbs</th>
				<th>Fat</th>
				<th>
					<button type="button" class="btn-icon variant-soft" on:click={addRaw}>
						<IconAdd />
					</button>
				</th>
			</tr>
		</thead>
		<tbody>
			{#each $rawIngredients as row, i (row.name)}
				<RawIngredientRow raw={row} on:remove={() => removeRaw(i)} />
			{/each}
		</tbody>
	</table>
</div>
