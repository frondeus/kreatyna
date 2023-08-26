<script lang="ts" context="module">
	export type MealWithDay = {
		name: string;
		recipe?: string;
	} & DayItem;
</script>

<script lang="ts">
	import DayPlanItem, { type DayItem } from "./DayPlanItem.svelte";
	import IconLink from '~icons/mdi/link-variant-plus';

    export let meal: MealWithDay;
</script>

<DayPlanItem item={meal}>
	<div class="min-w-0 whitespace-nowrap">
		<header class="flex">
			<span class="grow h5"> {meal.name} </span>
			<span role="link" class="cursor-pointer">
				<IconLink />
			</span>
		</header>
		{#if meal.recipe}
		<span class="truncate block">
				{meal.recipe}
		</span>
		{/if}
	</div>
</DayPlanItem>
