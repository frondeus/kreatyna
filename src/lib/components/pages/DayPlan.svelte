<script lang="ts">
	import MealPlanDay from '$components/molecules/MealPlanDay.svelte';
	import { daysInWeek, type WeekPlan } from '$shared/domains/plan/models';
	import { sortMeals } from '$shared/domains/plan/utils';

	export let weekPlan: WeekPlan;
	export let currentDate: Date;
	$: day = currentDate.getDay();
</script>

<MealPlanDay
	{currentDate}
	day={day + 1}
	year={weekPlan.year}
	week={weekPlan.week}
	name={daysInWeek[day].name + ' in week ' + weekPlan.week}
	meals={weekPlan.meals.filter((m) => m.day === day + 1).sort(sortMeals)}
	selected={new Map()}
	noCheckbox
/>
