<script context="module" lang="ts">
	export enum State {
		Default = 'default',
		Added = 'added'
	}
</script>

<script lang="ts">
	import IconPlusOutline from '~icons/mdi/plus-circle-outline';
	import IconPlus from '~icons/mdi/plus-circle';

	export let state: State = State.Default;

	const animationClasses = 'cursor-pointer hover:scale-150 transform transition duration-200';
	let icon = IconPlusOutline;
	let color = 'text-blue-300';
	$: {
		switch (state) {
			case State.Default:
				icon = IconPlusOutline;
				color = 'text-blue-300';
				break;
			case State.Added:
				icon = IconPlus;
				color = 'text-primary-500';
				break;
		}
	}
	$: klass = color + ' ' + animationClasses;
	$: isChecked = state !== State.Default;
</script>

<span class="text-2xl" role="checkbox" aria-checked={isChecked} on:click on:keypress tabindex="0">
	<svelte:component this={icon} class={klass} />
</span>
