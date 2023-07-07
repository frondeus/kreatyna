<script lang="ts">
	import { writable, derived, get, type Readable } from 'svelte/store';
	import type { Goal, Sum } from '$lib/types';
	import { derivedIngredient, derivedSum } from '$lib/stores';
	import type { RawIngredentsStore as RawIngredientsStore, IngredientsStore } from '$lib/stores';
	import IngredientRow from '$lib/components/Ingredient.svelte';
	import GoalRow from '$lib/components/Goal.svelte';
	import SumRow from '$lib/components/Sum.svelte';
	import rawIngredientsJson from '$lib/data/raw.json';

	export let data;

	let goal: Goal = {
		protein: writable(72.8),
		carbs: writable(92),
		fat: writable(28.7)
	};

	let rawIngredients: RawIngredientsStore = writable(
		rawIngredientsJson.map((entry) => ({
			name: writable(entry.name),
			container: writable(entry.container),
			protein: writable(entry.protein),
			carbs: writable(entry.carbs),
			fat: writable(entry.fat)
		}))
	);

	let ingredients: IngredientsStore = derived(rawIngredients, (rawIgredients) =>
		rawIgredients.map((raw) => {
			let amount = writable(get(raw.container));
			return {
				amount,
				data: derivedIngredient(amount, raw)
			};
		})
	);

	let proteinSum = derivedSum(ingredients, (data) => data.protein);
	let carbsSum = derivedSum(ingredients, (data) => data.carbs);
	let fatSum = derivedSum(ingredients, (data) => data.fat);

	let sum: Readable<Sum> = derived([proteinSum, carbsSum, fatSum], ([protein, carbs, fat]) => ({
		protein,
		carbs,
		fat
	}));
</script>

<div class="table-container">
	<table class="table">
		<thead>
			<tr>
				<th>Name</th>
				<th>Amount</th>
				<th>Proteins</th>
				<th>Carbs</th>
				<th>Fat</th>
			</tr>
		</thead>
		<tbody>
			{#each $ingredients as row (row.data)}
				<IngredientRow ingredient={row.amount} withData={row.data} />
			{/each}
		</tbody>
		<tfoot>
			<SumRow {sum} />
			<GoalRow {goal} />
		</tfoot>
	</table>
</div>

