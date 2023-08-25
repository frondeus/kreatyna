<script lang="ts">
	import MealPlanDay, { type MealWithDay } from '$components/molecules/MealPlanDay.svelte';

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

	const meals: MealWithDay[] = [
		...dayIndices.map((day) => ({
			day: { index: day },
			slot: { index: 2, size: 2 },
			name: 'Breakfast'
		})),
		...dayIndices.map((day) => ({
			day: { index: day },
			slot: { index: 6 },
			name: 'Snack'
		})),
		...dayIndices
			.filter((d) => !atWork.includes(d) && !weekend.includes(d))
			.map((day) => ({
				day: { index: day },
				slot: { index: 8 },
				name: 'Workout',
				variant: 'tertiary' // Workout
			})),
		...atWork.map((day) => ({
			day: { index: day },
			slot: { index: 16 },
			name: 'Workout',
			variant: 'tertiary' // Workout
		})),
		...dayIndices
			.filter((d) => d % 2 === 1)
			.map((day) => ({
				day: { index: day },
				slot: { index: 9, size: 1 },
				name: 'Lunch',
				variant: 'secondary' // Cook
			})),
		...dayIndices.map((day) => ({
			day: { index: day },
			slot: { index: 10, size: 2 },
			name: 'Lunch'
		})),
		...dayIndices.map((day) => ({
			day: { index: day },
			slot: { index: 14 },
			name: 'Desert'
		})),
		...dayIndices
			.filter((d) => d % 2 === 1)
			.map((day) => ({
				day: { index: day },
				slot: { index: 16, size: 1 },
				name: 'Dinner',
				variant: 'secondary' // Cook
			})),
		...dayIndices.map((day) => ({
			day: { index: day },
			slot: { index: 17, size: 2 },
			name: 'Dinner'
		}))
	].sort((a, b) => {
		if (a.day.index === b.day.index) {
			return a.slot.index - b.slot.index;
		}

		return a.day.index - b.day.index;
	});

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

<div class="lg:grid lg:grid-cols-7 lg:h-full">
	{#each days as day, idx}
		<MealPlanDay name={day.name} meals={mealsPerDay.get(idx + 1)} />
	{/each}
</div>
