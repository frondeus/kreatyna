<script lang="ts">
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

	//Group meals per day
	const mealsPerDay: Map<number, WeekPlanMeal[]> = weekPlan.meals.reduce((acc, meal) => {
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
</script>

<div class="space-y-2 lg:flex lg:flex-col lg:h-full p-2 card">
	<div class="max-lg:hidden card lg:flex-initial p-1">
		<ActionIconButton>
			<IconDice />
		</ActionIconButton>
	</div>
	<div class="lg:grid lg:grid-cols-7">
		{#each days as day, idx}
			<MealPlanDay name={day.name} meals={mealsPerDay.get(idx + 1)?.sort(sortItem)} />
		{/each}
	</div>
</div>
