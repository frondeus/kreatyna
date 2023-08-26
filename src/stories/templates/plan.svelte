<script lang="ts">
	import ActionIconButton from '$components/atoms/ActionIconButton.svelte';
	import MealPlanDay, { type MealWithDay } from '$components/molecules/MealPlanDay.svelte';
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

	const dayIndices = [...Array(7).keys()].map((day) => day + 1);
	const atWork = [4, 6];
	const weekend = [1, 7];

	export let meals: MealWithDay[] = [];

	console.log({ meals });
	//Group meals per day
	const mealsPerDay: Map<number, MealWithDay[]> = meals.reduce((acc, meal) => {
		const day = meal.day.index;
		if (!acc.has(day)) {
			acc.set(day, []);
		}

		acc.get(day)?.push(meal);

		return acc;
	}, new Map<number, MealWithDay[]>());
</script>

<div class="space-y-2 lg:flex lg:flex-col lg:h-full p-2 card overflow-hidden">
	<div class="max-lg:hidden card lg:flex-initial p-2">
		<ActionIconButton>
			<IconDice />
		</ActionIconButton>
	</div>
	<div class="lg:grid lg:grid-cols-7 overflow-hidden max-h-full">
		{#each days as day, idx}
			<MealPlanDay name={day.name} meals={mealsPerDay.get(idx + 1)} />
		{/each}
	</div>
</div>
