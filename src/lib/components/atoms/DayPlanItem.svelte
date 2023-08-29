<script lang="ts" context="module">
	import type { Size } from '$shared/domains/plan/models';

	export interface DayItem {
		day: Size; // From 1 to 7
		slot: Size; // From 1 to 12
		variant?: 'primary' | 'secondary' | 'surface';
	}
</script>

<script lang="ts">
	export let item: DayItem;

	let variant = 'primary';
	// $: daySize = meal.day.size ?? 1;
	$: slotSize = item.slot.size ?? 1;
	// $: variant = item.variant ?? 'primary';
	$: {
		switch (item.variant) {
			case 'primary':
				variant = 'primary';
				break;
			case 'secondary':
				variant = 'secondary';
				break;
			case 'surface':
				variant = 'surface';
				break;
		}
	}
</script>

<div
	class="
rounded-md
m-1
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
