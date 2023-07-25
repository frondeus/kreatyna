<script lang="ts">
	import { page } from '$app/stores';
	import { fromDB, type RawIngredient, type RawIngredientRow } from '$lib/types';
	import { modalStore, type AutocompleteOption, Autocomplete } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { derived, writable, type Readable, type Writable } from 'svelte/store';

	// export let form: SuperForm<AddRecipeSchema | EditRecipeSchema>;
	// export let idx: number;
	export let name: string;

	$: supabase = $page.data.supabase;
	const raw: Writable<RawIngredient[]> = writable([]);

	const autocomplete: Readable<AutocompleteOption[]> = derived(raw, ($raw) => {
		// return $raw.filter((row) => row.name.toLowerCase().includes(name.toLowerCase()));
		return $raw.map((r) => ({
			label: r.name,
			value: r.name,
			meta: r
		}));
	});

	onMount(async () => {
		const { data } = await supabase.from('raw').select('*');
		$raw = (data ?? []).map((r: RawIngredientRow) => fromDB.raw(undefined, r));
	});

	const onSelection = (event: CustomEvent<{ meta: RawIngredient }>) => {
		$modalStore?.[0]?.response?.(event.detail.meta);
		modalStore.close();
		// parent.onClose(666);
	};
</script>

{#if $modalStore}
	<div class="card p-4 shadow-xl space-y-4">
		<h2 class="h2">Search for raw ingredient</h2>

		<input class="input" type="search" bind:value={name} placeholder="Search for raw ingredient" />

		<div class="card w-ful max-h-48 p-4 overflow-y-auto" tabindex="-1">
			<Autocomplete bind:input={name} options={$autocomplete} on:selection={onSelection} />
		</div>
	</div>
{/if}
