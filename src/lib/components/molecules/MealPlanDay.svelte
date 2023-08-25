<script lang="ts" context="module">
	import type { Meal } from '$components/atoms/MealPlanItem.svelte';

	export type MealWithDay = {
		name: string;
	} & Meal;
</script>

<script lang="ts">
	import MealPlanItem from '$components/atoms/MealPlanItem.svelte';

	export let name: string;
	export let meals: MealWithDay[] = [];

	export let maxSize = meals.reduce((acc, meal) => acc + (meal.slot.size ?? 1), 0);
</script>

<div class="bg-surface-800 even:bg-surface-700 text-center my-4 lg:my-0 lg:h-full">
	<div class="p-2 xl:p-4 lg:flex-initial">
		{name}
	</div>

	<!-- <div class="grid auto-cols-fr grid-rows-{maxSize} lg:grid-rows-18 justify-between content-between"> -->
	<div class="grid grid-rows-{maxSize} xl:grid-rows-18 py-4 lg:py-0">
		{#each meals as meal}
			<MealPlanItem {meal}>
				{meal.name}
			</MealPlanItem>
		{/each}
	</div>
</div>
