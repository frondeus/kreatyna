<script lang="ts">
	import WeekPlanComponent from '$components/pages/WeekPlan.svelte';
	import type { WeekPlan } from '$shared/domains/plan/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { weekPlan }: { weekPlan: WeekPlan } = data;
	$: ({ weekPlan } = data);
	const date = new Date();
	date.setHours(0, 0, 0, 0);
</script>

<WeekPlanComponent {weekPlan} currentDate={date} />
