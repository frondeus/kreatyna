<script lang="ts">
	import {
		Autocomplete,
		popup,
		type AutocompleteOption,
		type PopupSettings
	} from '@skeletonlabs/skeleton';

	let search = '';
	export let options: AutocompleteOption[];

	const randomId = () => Math.random().toString(36).substring(7);
	const targetId = 'popupAutocomplete' + randomId();
	const searchPopup: PopupSettings = {
		event: 'focus-click',
		target: targetId,
		placement: 'bottom'
	};
</script>

<input
	class="input"
	title="Search"
	type="text"
	placeholder="Search..."
	bind:value={search}
	use:popup={searchPopup}
/>

<div class="card p-4 z-10" data-popup={targetId}>
	<Autocomplete bind:input={search} {options} on:selection />
</div>
