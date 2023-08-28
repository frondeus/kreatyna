<script lang="ts">
	import DayPlanMeal from '$components/atoms/DayPlanMeal.svelte';
	import type { WeekPlanMeal } from '$shared/domains/plan/models';

	export let name: string;
	export let meals: WeekPlanMeal[] = [];

	export let maxSize = meals.reduce((acc, meal) => acc + (meal.size ?? 1), 0);
</script>

<div class="bg-surface-800 even:bg-surface-700 text-center my-4 lg:my-0 h-full flex flex-col">
	<div class="p-2 xl:p-4 lg:flex-initial">
		{name}
	</div>

	<div
		class="grid h-full grow grid-cols-1 grid-rows-{maxSize} xl:grid-rows-18 py-4 lg:py-0 justify-items-stretch"
	>
		{#each meals as meal}
			<DayPlanMeal {meal} />
		{/each}
	</div>
</div>
