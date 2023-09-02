<script lang="ts">
	import DayPlanMeal from '$components/atoms/DayPlanMeal.svelte';
	import type { WeekPlanMeal } from '$shared/domains/plan/models';
	import { getDateOfIsoWeek } from '$shared/utils';

	export let name: string;
	export let meals: WeekPlanMeal[] = [];
	export let currentDate: Date;
	export let year: number;
	export let week: number;
	export let day: number;
	export let selected: Map<number, boolean>;
	export let noCheckbox = false;

	$: mealsIds = meals.map((m) => m.id);

	$: maybeSelectedMealsInThisColumn = Array.from(selected.entries()).filter(([id, _]) =>
		mealsIds.includes(id)
	);

	$: maybeSelectedMealsInThisColumnBool = maybeSelectedMealsInThisColumn.map(
		([_, selected]) => selected
	);

	$: selectedAny = maybeSelectedMealsInThisColumnBool.some((v) => v);
	$: selectedAll =
		maybeSelectedMealsInThisColumnBool.every((v) => v) &&
		maybeSelectedMealsInThisColumnBool.length > 0;
	$: isIndeterminate = selectedAny && !selectedAll;

	$: dayDate = getDateOfIsoWeek(week, year, day - 2);
	let variant: 'primary' | 'secondary' | 'surface';
	$: {
		/// Is the date in past ignoring hours and minutes
		const isInPast = dayDate < currentDate;
		const isPresent = dayDate === currentDate;
		const isFuture = dayDate > currentDate;

		if (isInPast) variant = 'surface';
		if (isPresent) variant = 'primary';
		if (isFuture) variant = 'secondary';
	}
	$: maxSize = meals.reduce((acc, meal) => acc + (meal.size ?? 1), 0);
	const onSelectAll = () => {
		for (const id of mealsIds) {
			selected.set(id, !selectedAll);
		}
		selected = selected;
	};
</script>

<div class="bg-surface-800 even:bg-surface-700 text-center lg:my-0 h-full flex flex-col">
	<div class="p-2 xl:py-4 xl:px-3 lg:flex-initial lg:flex lg:justify-between">
		<span />

		{name}

		{#if !noCheckbox}
			<input
				type="checkbox"
				class="checkbox max-lg:hidden"
				checked={selectedAll}
				bind:indeterminate={isIndeterminate}
				on:change={onSelectAll}
			/>
		{:else}
			<span />
		{/if}
	</div>

	<div
		class="grid h-full grow grid-cols-1 grid-rows-{maxSize} xl:grid-rows-18 py-4 lg:py-0 justify-items-stretch"
	>
		{#each meals as meal (meal.id)}
			<DayPlanMeal {meal} {variant} bind:selected {noCheckbox} />
		{/each}
	</div>
</div>
