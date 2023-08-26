<script lang="ts">
	import DayPlanMeal, { type MealWithDay } from '$components/atoms/DayPlanMeal.svelte';

	export let name: string;
	export let meals: MealWithDay[] = [];

	export let maxSize = meals.reduce((acc, meal) => acc + (meal.slot.size ?? 1), 0);
</script>

<div class="bg-surface-800 even:bg-surface-700 text-center my-4 lg:my-0 h-full overflow-auto">
	<div class="p-2 xl:p-4 lg:flex-initial">
		{name}
	</div>

	<div class="grid grid-cols-1 grid-rows-{maxSize} xl:grid-rows-18 py-4 lg:py-0 overflow-auto">
		{#each meals as meal}
			<DayPlanMeal {meal}/>
		{/each}
	</div>
</div>
