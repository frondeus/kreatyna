<script lang="ts">
	import type { WeekPlanMeal } from '$shared/domains/plan/models';
	import DayPlanItem from './DayPlanItem.svelte';
	import IconLink from '~icons/mdi/link-variant-plus';

	export let meal: WeekPlanMeal;

	$: item = {
		day: { index: meal.day },
		slot: { index: meal.slot, size: meal.size },
		variant: 'primary' as const
	};
</script>

<DayPlanItem {item}>
	<div class="min-w-0 flex h-full divide-solid divide-x-2 divide-emerald-500">
		<div class="flex items-center">
			<span class="px-2"> {meal.name} </span>
		</div>
		<div class="flex justify-end items-center grow p-2">
			{#if meal.recipe}
				<a
					class="px-2 grow truncate block whitespace-break-spaces text-xs"
					href={`/browse/recipe/${meal.recipe.id}`}
				>
					{meal.recipe.name}
				</a>
			{/if}
			<div class="self-start justify-self-end items-center justify-end flex gap-2">
				{#if !meal.recipe}
					<span role="link" class="cursor-pointer text-right text-xs">
						<IconLink />
					</span>
				{/if}
				<input type="checkbox" class="checkbox" />
			</div>
		</div>
	</div>
</DayPlanItem>
