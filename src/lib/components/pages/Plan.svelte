<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import ActionIconButton from '$components/atoms/ActionIconButton.svelte';
	import MealPlanDay from '$components/molecules/MealPlanDay.svelte';
	import type { WeekPlan, WeekPlanMeal } from '$shared/domains/plan/models';
	import IconDice from '~icons/mdi/dice-5';

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
		console.log('refreshing');
		await invalidateAll();
	};
</script>

<div class="space-y-2 lg:flex lg:flex-col lg:h-full p-2 card">
	<div class="max-lg:hidden card lg:flex-initial p-1">
		<ActionIconButton on:click={handlePopulate} on:keypress={handlePopulate}>
			<IconDice />
		</ActionIconButton>
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
			/>
		{/each}
	</div>
</div>
