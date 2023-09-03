<script lang="ts">
	import type { RawIngredient } from '$lib/shared/domains/raw/models';
	import { modalStore, type AutocompleteOption, Autocomplete } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { derived, writable, type Readable, type Writable } from 'svelte/store';

	export let name: string;

	const raw: Writable<RawIngredient[]> = writable([]);

	const autocomplete: Readable<AutocompleteOption[]> = derived(raw, ($raw) => {
		return $raw.map((r) => ({
			label: r.name,
			value: r.name,
			meta: r
		}));
	});

	onMount(async () => {
		const response = await fetch('/browse/raw/search', {
			method: 'GET'
		});

		$raw = await response.json();
	});

	const onSelection = (event: CustomEvent<AutocompleteOption>) => {
		$modalStore?.[0]?.response?.(event.detail.meta);
		modalStore.close();
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
