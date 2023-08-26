<script lang="ts" context="module">
	export interface Size {
		index: number;
		size?: number;
	}
	export interface DayItem {
		day: Size; // From 1 to 7
		slot: Size; // From 1 to 12
		variant?: 'primary' | 'secondary' | 'tertiary';
	}
</script>

<script lang="ts">
	export let item: DayItem;

	// $: daySize = meal.day.size ?? 1;
	$: slotSize = item.slot.size ?? 1;
	$: variant = item.variant ?? 'primary';
</script>

<div
	class="
rounded-md
m-1
p-1
variant-glass-{variant} variant-ringed-{variant}
row-span-{slotSize}
xl:row-start-{item.slot.index}
overflow-hidden
min-w-0
min-h-0
"
>
	<!-- {meal.slot.index} - {slotSize} -->
	<slot />
</div>
