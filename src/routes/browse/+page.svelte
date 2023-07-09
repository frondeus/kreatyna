<script lang="ts">
	import RawBrowser from './RawList.svelte';
	import { Tab } from '@skeletonlabs/skeleton';
	import { TabGroup } from '@skeletonlabs/skeleton';

	let tabSet = 0;

	export let data;
</script>

<TabGroup>
	<Tab bind:group={tabSet} name="tab1" value={0}>
		<span>Raw ingredients</span>
	</Tab>
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tabSet === 0}
			<RawBrowser {data} />
		{/if}
	</svelte:fragment>
</TabGroup>
