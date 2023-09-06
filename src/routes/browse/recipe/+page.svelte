<script lang="ts">
	import type { PageData } from './$types';
	import IconAdd from '~icons/mdi/add';
	import Table from '$components/atoms/tables/Table.svelte';
	import TableRow from '$components/atoms/tables/TableRow.svelte';
	import { writable, type Writable } from 'svelte/store';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import InputChipSearch from '$components/molecules/InputChipSearch.svelte';
	import { onMount } from 'svelte';
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import type { RecipeFilter } from '$shared/domains/recipe/models';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let autocomplete: Writable<AutocompleteOption[]> = writable([]);
	let filters: Writable<RecipeFilter[]> = writable([]);

	let { recipes, related } = data;
	$: ({ recipes, related } = data);

	onMount(async () => {
		const tags = await related.tags;
		$autocomplete = (tags ?? []).map((r) => ({
			label: r.category + ': ' + r.name,
			value: r.id,
			meta: r
		}));

		filters.subscribe((filters) => {
			let query = new URLSearchParams($page.url.searchParams.toString());
			query.delete('tag');
			for (const tag of filters) {
				query.append('tag', tag.tagId.toString());
			}
			goto(`?${query.toString()}`);
		});
	});
</script>

<svelte:head>Kreatyna - Browse Recipes</svelte:head>

<div class="space-y-4">
	<InputChipSearch
		options={$autocomplete}
		on:select={(e) => {
			$filters.push({ tagId: e.detail.id });
			$filters = $filters;
		}}
		on:remove={(e) => {
			$filters = $filters.filter((f) => f.tagId !== e.detail.id);
		}}
	/>

	<Table columns={[{ name: 'Name' }]}>
		<svelte:fragment slot="headerActions">
			<ActionIconButton href="./recipe/create"><IconAdd /></ActionIconButton>
		</svelte:fragment>

		{#if !recipes}
			Loading...
		{:else}
			{#each recipes as row (row.id)}
				<TableRow id={row.id} href="./recipe" name={row.name} editable deletable />
			{/each}
		{/if}
	</Table>
</div>
