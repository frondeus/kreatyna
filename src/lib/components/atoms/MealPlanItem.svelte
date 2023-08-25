<script lang="ts" context="module">
	export interface Size {
		index: number;
		size?: number;
	}
	export interface Meal {
		day: Size; // From 1 to 7
		slot: Size; // From 1 to 12
		variant?: 'primary' | 'secondary' | 'tertiary';
	}
</script>

<script lang="ts">
	export let meal: Meal;

	// $: daySize = meal.day.size ?? 1;
	$: slotSize = meal.slot.size ?? 1;
	$: variant = meal.variant ?? 'primary';
</script>

<div
	class="
rounded-md
m-1
p-1
variant-glass-{variant} variant-ringed-{variant}
row-span-{slotSize}
xl:row-start-{meal.slot.index}
"
>
	<!-- {meal.slot.index} - {slotSize} -->
	<slot />
</div>
