<script>
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import { onMount } from 'svelte';
	import ArrowLeft from '~icons/mdi/arrow-left';
	import Navigation from '$components/molecules/Navigation.svelte';
	import { navigating } from '$app/stores';
	import { drawerStore } from '@skeletonlabs/skeleton';

	const onClose = () => {
		drawerStore.close();
	};

	onMount(() => {
		navigating.subscribe((n) => {
			if (n !== null) {
				onClose();
			}
		});
	});
</script>

<ActionIconButton on:click={onClose} on:keypress={onClose}>
	<ArrowLeft />
</ActionIconButton>

<Navigation />
