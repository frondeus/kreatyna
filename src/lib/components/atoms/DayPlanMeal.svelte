<script lang="ts">
	import type { MealForDay } from '$shared/domains/plan/models';
	import DayPlanItem from './DayPlanItem.svelte';
	import IconLink from '~icons/mdi/link-variant-plus';

	export let meal: MealForDay;

	$: item = {
		day: { index: meal.day },
		slot: meal.slot,
		variant: 'primary' as const
	};
</script>

<DayPlanItem {item}>
	<div class="min-w-0">
		<header class="flex">
			<span class="grow h4"> {meal.name} </span>
			{#if !meal.recipe}
				<span role="link" class="cursor-pointer">
					<IconLink />
				</span>
			{/if}
		</header>
		{#if meal.recipe}
			<a class="truncate block whitespace-break-spaces" href={`/browse/recipe/${meal.recipe.id}`}>
				{meal.recipe.name}
			</a>
		{/if}
	</div>
</DayPlanItem>
