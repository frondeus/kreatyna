<script context="module" lang="ts">
	export interface WeekPlanEvent {
		year: number;
		week: number;
	}

	export interface WeekPlanMealsEvent extends WeekPlanEvent {
		meals: number[];
	}
</script>

<script lang="ts">
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import MealPlanDay from '$components/molecules/MealPlanDay.svelte';
	import { daysInWeek, type WeekPlan, type WeekPlanMeal } from '$shared/domains/plan/models';
	import { sortMeals } from '$shared/domains/plan/utils';
	import { createEventDispatcher } from 'svelte';
	import IconDice from '~icons/mdi/dice-5';
	import IconUnlink from '~icons/mdi/link-variant-off';
	import IconBasket from '~icons/mdi/basket-plus-outline';

	export let weekPlan: WeekPlan;
	export let currentDate: Date;

	//Group meals per day
	$: mealsPerDay = weekPlan.meals.reduce((acc, meal) => {
		const day = meal.day;
		if (!acc.has(day)) {
			acc.set(day, []);
		}

		acc.get(day)?.push(meal);

		return acc;
	}, new Map<number, WeekPlanMeal[]>());

	const setSelectedMeals = (weekPlan: WeekPlan) => {
		return new Map(weekPlan.meals.map((m) => [m.id, false as boolean] as const));
	};

	let selectedMeals = setSelectedMeals(weekPlan);

	$: selectedMealsArrayOfIds = Array.from(selectedMeals.entries())
		.filter(([, selected]) => selected)
		.map(([id]) => id);

	$: selectedMealsArray = weekPlan.meals.filter((m) => selectedMealsArrayOfIds.includes(m.id));

	$: selectedAny = Array.from(selectedMeals.values()).some((v) => v);
	$: selectedAll = Array.from(selectedMeals.values()).every((v) => v);
	$: isIndeterminate = selectedAny && !selectedAll;

	const onSelectAll = () => {
		if (selectedAll) {
			selectedMeals = new Map(weekPlan.meals.map((m) => [m.id, false] as const));
		} else {
			selectedMeals = new Map(weekPlan.meals.map((m) => [m.id, true] as const));
		}
	};

	const dispatch = createEventDispatcher();

	const handlePopulate = async () => {
		dispatch('populate', { year: weekPlan.year, week: weekPlan.week });
	};

	const handleUnlink = async () => {
		dispatch('unlink', {
			year: weekPlan.year,
			week: weekPlan.week,
			meals: selectedMealsArrayOfIds
		});
		selectedMeals = setSelectedMeals(weekPlan);
	};

	const handleShop = async () => {
		dispatch('shop', {
			year: weekPlan.year,
			week: weekPlan.week,
			meals: selectedMealsArrayOfIds
		});
		selectedMeals = setSelectedMeals(weekPlan);
	};
</script>

<div class="space-y-2 lg:flex lg:flex-col lg:h-full p-2 card">
	<div class="max-lg:hidden card lg:flex-initial p-1 flex items-center gap-2 justify-end px-2">
		{#if weekPlan.meals.some((meal) => meal.recipe === null)}
			<ActionIconButton title="Populate" on:click={handlePopulate} on:keypress={handlePopulate}>
				<IconDice />
			</ActionIconButton>
		{/if}
		{#if selectedMealsArray.some((meal) => meal.recipe !== null)}
			<ActionIconButton title="Unlink" on:click={handleUnlink} on:keypress={handleUnlink}>
				<IconUnlink />
			</ActionIconButton>

			<ActionIconButton title="Shop" on:click={handleShop} on:keypress={handleShop}>
				<IconBasket />
			</ActionIconButton>
		{/if}
		<input
			type="checkbox"
			class="checkbox m-2"
			checked={selectedAll}
			bind:indeterminate={isIndeterminate}
			on:change={onSelectAll}
		/>
	</div>
	<div class="lg:grid lg:grid-cols-7">
		{#each daysInWeek as day, idx (idx)}
			<MealPlanDay
				{currentDate}
				day={idx + 1}
				year={weekPlan.year}
				week={weekPlan.week}
				name={day.name}
				meals={mealsPerDay.get(idx + 1)?.sort(sortMeals)}
				bind:selected={selectedMeals}
			/>
		{/each}
	</div>
</div>
