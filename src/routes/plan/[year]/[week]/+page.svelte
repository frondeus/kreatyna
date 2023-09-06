<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import WeekPlanComponent, {
		type WeekPlanEvent,
		type WeekPlanMealsEvent
	} from '$components/pages/WeekPlan.svelte';
	import type { WeekPlan } from '$shared/domains/plan/models';
	import type { PageData } from './$types';

	export let data: PageData;
	let { weekPlan }: { weekPlan: WeekPlan } = data;
	$: ({ weekPlan } = data);
	const date = new Date();
	date.setHours(0, 0, 0, 0);

	const onPopulate = async (event: CustomEvent<WeekPlanEvent>) => {
		const res = await fetch('/plan/populate', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(event.detail)
		});
		if (res.status !== 200) {
			console.error('Error populating plan');
			return;
		}
		await invalidateAll();
	};

	const onUnlink = async (event: CustomEvent<WeekPlanMealsEvent>) => {
		const res = await fetch('/plan/unlink', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(event.detail)
		});
		if (res.status !== 200) {
			console.error('Error unlinking meals');
			return;
		}
		await invalidateAll();
	};

	const onShop = async (event: CustomEvent<WeekPlanMealsEvent>) => {
		const res = await fetch('/plan/shop', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(event.detail)
		});
		if (res.status !== 200) {
			console.error('Error unlinking meals');
			return;
		}
		goto('/shop');
	};
</script>

<svelte:head>
	<title>Kreatyna - Weekly Meal Plan</title>
</svelte:head>

<WeekPlanComponent
	{weekPlan}
	currentDate={date}
	on:populate={onPopulate}
	on:unlink={onUnlink}
	on:shop={onShop}
/>
