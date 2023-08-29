<script lang="ts">
	import Plan from '$components/pages/Plan.svelte';
	import type { WeekPlan } from '$shared/domains/plan/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { weekPlan }: { weekPlan: WeekPlan } = data;
	$: ({ weekPlan } = data);
	const date = new Date();
	/// Set to midnight
	date.setHours(0, 0, 0, 0);
</script>

<Plan {weekPlan} currentDate={date} />
