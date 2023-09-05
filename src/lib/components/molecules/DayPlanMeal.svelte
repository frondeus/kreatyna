<script lang="ts">
	import type { WeekPlanMeal } from '$shared/domains/plan/models';
	import type { ChangeEventHandler } from 'svelte/elements';
	import DayPlanItem from '../atoms/DayPlanItem.svelte';
	import IconLink from '~icons/mdi/link-variant-plus';

	export let meal: WeekPlanMeal;
	export let variant: 'primary' | 'secondary' | 'surface';
	export let selected: Map<number, boolean>;
	export let noCheckbox = false;

	const onInput = (({ currentTarget }) => {
		const { checked } = currentTarget;
		selected.set(meal.id, checked);
		selected = selected;
	}) satisfies ChangeEventHandler<HTMLInputElement>;

	$: item = {
		day: { index: meal.day },
		slot: { index: meal.slot, size: meal.size },
		variant
	};

	let divideColor = 'divide-emerald-500';
	switch (variant) {
		case 'primary':
			divideColor = 'divide-emerald-500';
			break;
		case 'secondary':
			divideColor = 'divide-indigo-500';
			break;
		case 'surface':
			divideColor = 'divide-surface-500';
			break;
	}
</script>

<DayPlanItem {item}>
	<div class="min-w-0 flex h-full divide-solid divide-x-2 {divideColor}">
		<div class="flex items-center basis-1/4">
			<span class="px-2"> {meal.name} </span>
		</div>
		<div class="flex justify-end items-center grow p-2 basis-3/4">
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
				{#if !noCheckbox}
					<input
						type="checkbox"
						class="checkbox max-lg:hidden"
						checked={selected.get(meal.id) ?? false}
						on:change={onInput}
					/>
				{/if}
			</div>
		</div>
	</div>
</DayPlanItem>
