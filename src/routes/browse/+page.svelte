<script lang="ts">
	import { Tab } from '@skeletonlabs/skeleton';
	import { TabGroup } from '@skeletonlabs/skeleton';
	import RawList from './RawList.svelte';
	import RecipiesList from './RecipiesList.svelte';

	enum TabSet {
		Raw = 0,
		Recipies = 1,
	}
	let tabSet: TabSet = TabSet.Recipies;

	export let data;
</script>

<TabGroup>
	<Tab bind:group={tabSet} name="recipies" value={TabSet.Recipies}> <span>Recipies</span> </Tab>
	<Tab bind:group={tabSet} name="raw" value={TabSet.Raw}> <span>Raw ingredients</span> </Tab>
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tabSet === TabSet.Raw}
			<RawList {data} />
		{:else if tabSet === TabSet.Recipies}
			<RecipiesList {data} />
		{/if}
	</svelte:fragment>
</TabGroup>
