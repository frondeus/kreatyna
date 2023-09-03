<script lang="ts">
	import {
		InputChip,
		type AutocompleteOption,
		type PopupSettings,
		Autocomplete,
		popup
	} from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';

	export let options: AutocompleteOption[];

	let search = '';
	let filters: string[] = [];
	let metas: unknown[] = [];

	$: filteredOptions = options.filter((option) => !metas.includes(option.meta));

	const randomId = () => Math.random().toString(36).substring(7);
	const targetId = 'popupAutocomplete' + randomId();
	const searchPopup: PopupSettings = {
		event: 'focus-click',
		target: targetId,
		placement: 'bottom'
	};

	$: whitelist = options.map((option) => {
		return option.label;
	});

	const dispatcher = createEventDispatcher();
	const onSelect = (event: CustomEvent<AutocompleteOption>) => {
		filters = [...filters, event.detail.label];
		metas = [...metas, event.detail.meta];
		dispatcher('select', event.detail.meta);
	};
	const onAdd = (event: CustomEvent<{ chipIndex: number; chipValue: string }>) => {
		const { chipIndex, chipValue } = event.detail;
		const option = options.find((option) => {
			return option.label === chipValue;
		});
		if (option) {
			metas = [...metas.slice(0, chipIndex), option.meta, ...metas.slice(chipIndex)];
			dispatcher('select', option.meta);
		}
	};
	const onRemove = (event: CustomEvent<{ chipIndex: number }>) => {
		const deleted = metas.splice(event.detail.chipIndex, 1);
		metas = metas;
		dispatcher('remove', deleted[0]);
	};
</script>

<div use:popup={searchPopup}>
	<InputChip
		{whitelist}
		bind:input={search}
		bind:value={filters}
		name="filters"
		placeholder="Search..."
		on:remove={onRemove}
		on:add={onAdd}
	/>
</div>

<div class="card p-4 z-10" data-popup={targetId}>
	<Autocomplete bind:input={search} options={filteredOptions} on:selection={onSelect} />
</div>
