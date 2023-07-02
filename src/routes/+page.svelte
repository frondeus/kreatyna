<script lang="ts">
	import { writable, derived, get, type Readable } from 'svelte/store';
	import type { Goal, Sum } from '../types';
	import { derivedIngredient, derivedSum } from '../stores';
	import type { RawIngredentsStore as RawIngredientsStore, IngredientsStore } from '../stores';
	import IngredientRow from '../components/Ingredient.svelte';
	import GoalRow from '../components/Goal.svelte';
	import SumRow from '../components/Sum.svelte';
	import RawIngredientRow from '../components/RawIngredient.svelte';

	let goal: Goal = {
		protein: writable(72.8),
		carbs: writable(92),
		fat: writable(28.7)
	};

	let rawIngredients: RawIngredientsStore = writable([
		{
			name: writable('Chicken'),
			container: writable(100),
			protein: writable(21.5),
			carbs: writable(0.1),
			fat: writable(6.9)
		},
		{
			name: writable('Avocado'),
			container: writable(100),
			protein: writable(1.6),
			carbs: writable(0.5),
			fat: writable(15.1)
		},
		{
			name: writable('Rye Bread'),
			container: writable(100),
			protein: writable(5.1),
			carbs: writable(36.5),
			fat: writable(1.4)
		}
	]);

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

<div class="container mx-auto p-8 space-y-8">
	<div class="table-container">
		<table class="table table-hover">
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

	<h3 class="h3">Raw ingredients</h3>

	<div class="table-container">
		<!-- Native Table Element -->
		<table class="table table-hover">
			<thead>
				<tr>
					<th>Name</th>
					<th>Container</th>
					<th>Proteins</th>
					<th>Carbs</th>
					<th>Fat</th>
					<th>
						<button type="button" class="btn-icon variant-filled" on:click={addRaw}>add</button>
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
</div>
