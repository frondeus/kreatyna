<script lang="ts">
	import Plan from '$components/pages/Plan.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let { meals } = data;
	$: ({ meals } = data);
</script>

<Plan {meals} />
