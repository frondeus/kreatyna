<script lang="ts">
	import type { TagWithCategory } from '$shared/domains/tags/models';
	import { Autocomplete, type AutocompleteOption } from '@skeletonlabs/skeleton';
	import { createEventDispatcher, onMount } from 'svelte';
	import { derived, writable, type Readable, type Writable } from 'svelte/store';

	let newTag = '';

	const tags: Writable<TagWithCategory[]> = writable([]);

	const autocomplete: Readable<AutocompleteOption[]> = derived(tags, ($tags) => {
		return $tags.map((t) => ({
			label: `${t.category.name}:${t.name}`,
			value: t.id,
			meta: t
		}));
	});

	onMount(async () => {
		const response = await fetch('/browse/tag/search', {
			method: 'GET'
		});

		$tags = await response.json();
	});

	const dispatch = createEventDispatcher();
	const onSelection = (event: CustomEvent<{ meta: TagWithCategory }>) => {
		dispatch('add', event.detail.meta);
	};
</script>

<input class="input" title="Add Tag" type="text" placeholder="Add Tag" bind:value={newTag} />

<Autocomplete bind:input={newTag} options={$autocomplete} on:selection={onSelection} />
