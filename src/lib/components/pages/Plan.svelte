<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import ActionIconButton from '$components/atoms/ActionIconButton.svelte';
	import MealPlanDay from '$components/molecules/MealPlanDay.svelte';
	import type { WeekPlan, WeekPlanMeal } from '$shared/domains/plan/models';
	import IconDice from '~icons/mdi/dice-5';
	import IconUnlink from '~icons/mdi/link-variant-off';

	interface Day {
		name: string;
	}

	const days: Day[] = [
		{
			name: 'Sunday'
		},
		{
			name: 'Monday'
		},
		{
			name: 'Tuesday'
		},
		{
			name: 'Wednesday'
		},
		{
			name: 'Thursday'
		},
		{
			name: 'Friday'
		},
		{
			name: 'Saturday'
		}
	];

	export let weekPlan: WeekPlan;
	export let currentDate: Date;

	//Group meals per day
	$: mealsPerDay = weekPlan.meals.reduce((acc, meal) => {
		const day = meal.day;
		if (!acc.has(day)) {
			acc.set(day, []);
		}

		acc.get(day)?.push(meal);

		return acc;
	}, new Map<number, WeekPlanMeal[]>());

	const setSelectedMeals = (weekPlan: WeekPlan) => {
		return new Map(weekPlan.meals.map((m) => [m.id, false as boolean] as const));
	};

	let selectedMeals = setSelectedMeals(weekPlan);

	$: selectedMealsArrayOfIds = Array.from(selectedMeals.entries())
		.filter(([, selected]) => selected)
		.map(([id]) => id);

	$: selectedMealsArray = weekPlan.meals.filter((m) => selectedMealsArrayOfIds.includes(m.id));

	$: selectedAny = Array.from(selectedMeals.values()).some((v) => v);
	$: selectedAll = Array.from(selectedMeals.values()).every((v) => v);
	$: isIndeterminate = selectedAny && !selectedAll;

	const onSelectAll = () => {
		if (selectedAll) {
			selectedMeals = new Map(weekPlan.meals.map((m) => [m.id, false] as const));
		} else {
			selectedMeals = new Map(weekPlan.meals.map((m) => [m.id, true] as const));
		}
	};

	const sortItem = (a: WeekPlanMeal, b: WeekPlanMeal): number => {
		if (a.day === b.day) {
			return a.slot - b.slot;
		}

		return a.day - b.day;
	};

	const handlePopulate = async () => {
		const res = await fetch('/plan/populate', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				year: weekPlan.year,
				week: weekPlan.week
			})
		});
		if (res.status !== 200) {
			console.error('Error populating plan');
			return;
		}
		await invalidateAll();
	};

	const handleUnlink = async () => {
		const res = await fetch('/plan/unlink', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				year: weekPlan.year,
				week: weekPlan.week,
				meals: selectedMealsArrayOfIds
			})
		});
		if (res.status !== 200) {
			console.error('Error unlinking meals');
			return;
		}
		await invalidateAll();
		selectedMeals = setSelectedMeals(weekPlan);
	};
</script>

<div class="space-y-2 lg:flex lg:flex-col lg:h-full p-2 card">
	<div class="max-lg:hidden card lg:flex-initial p-1 flex items-center gap-2 justify-end px-2">
		{#if weekPlan.meals.some((meal) => meal.recipe === null)}
			<ActionIconButton title="Populate" on:click={handlePopulate} on:keypress={handlePopulate}>
				<IconDice />
			</ActionIconButton>
		{/if}
		{#if selectedMealsArray.some((meal) => meal.recipe !== null)}
			<ActionIconButton title="Unlink" on:click={handleUnlink} on:keypress={handleUnlink}>
				<IconUnlink />
			</ActionIconButton>
		{/if}
		<input
			type="checkbox"
			class="checkbox m-2"
			checked={selectedAll}
			bind:indeterminate={isIndeterminate}
			on:change={onSelectAll}
		/>
	</div>
	<div class="lg:grid lg:grid-cols-7">
		{#each days as day, idx (idx)}
			<MealPlanDay
				{currentDate}
				day={idx + 1}
				year={weekPlan.year}
				week={weekPlan.week}
				name={day.name}
				meals={mealsPerDay.get(idx + 1)?.sort(sortItem)}
				bind:selected={selectedMeals}
			/>
		{/each}
	</div>
</div>
