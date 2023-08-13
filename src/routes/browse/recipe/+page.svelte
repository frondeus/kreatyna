<script lang="ts">
	import type { PageData } from './$types';
	import IconAdd from '~icons/mdi/add';
	import Table from '$components/atoms/tables/Table.svelte';
	import TableRow from '$components/atoms/tables/TableRow.svelte';
	import { writable, type Writable } from 'svelte/store';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import InputChipSearch from '$components/molecules/InputChipSearch.svelte';
	import { onMount } from 'svelte';
	import ActionIconButton from '$components/atoms/ActionIconButton.svelte';

	export let data: PageData;

	let { recipies, related } = data;
	let autocomplete: Writable<AutocompleteOption[]> = writable([]);

	$: ({ recipies, related } = data);
	onMount(async () => {
		const tags = await related.tags;
		$autocomplete = (tags ?? []).map((r) => ({
			label: r.category + ': ' + r.name,
			value: r.id,
			meta: r
		}));
	});
</script>

<div class="space-y-4">
	<InputChipSearch
		options={$autocomplete}
		on:select={(e) => {
			console.log('select', e.detail);
		}}
		on:remove={(e) => {
			console.log('remove', e.detail);
		}}
	/>

	<Table columns={[{ name: 'Name' }]}>
		<svelte:fragment slot="headerActions">
			<ActionIconButton href="./recipe/create"><IconAdd /></ActionIconButton>
		</svelte:fragment>

		{#if !recipies}
			Loading...
		{:else}
			{#each recipies as row (row.id)}
				<TableRow id={row.id} href="./recipe" name={row.name} editable deletable />
			{/each}
		{/if}
	</Table>
</div>
