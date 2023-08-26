<script lang="ts">
	import type { MealForDay } from "$shared/domains/plan/models";
	import DayPlanItem, { type DayItem } from "./DayPlanItem.svelte";
	import IconLink from '~icons/mdi/link-variant-plus';

    export let meal: MealForDay;

	$: item = {
		day: { index: meal.day },
		slot: meal.slot,
		variant: 'primary' as const
	};
</script>

<DayPlanItem {item}>
	<div class="min-w-0 whitespace-nowrap">
		<header class="flex">
			<span class="grow h5"> {meal.name} </span>
			<span role="link" class="cursor-pointer">
				<IconLink />
			</span>
		</header>
		{#if meal.recipe}
		<span class="truncate block">
			{meal.recipe.name}
		</span>
		{/if}
	</div>
</DayPlanItem>
