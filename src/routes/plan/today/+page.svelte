<script lang="ts">
	import DayPlan from '$components/pages/DayPlan.svelte';
	import type { WeekPlan } from '$shared/domains/plan/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { weekPlan }: { weekPlan: WeekPlan } = data;
	$: ({ weekPlan } = data);

	const date = new Date();
	date.setHours(0, 0, 0, 0);
</script>

<svelte:head>
	<title>Kreatyna - Daily Meal Plan</title>
</svelte:head>

<DayPlan currentDate={date} {weekPlan} />
