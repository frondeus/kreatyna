<script context="module" lang="ts">
	export enum State {
		Processing = 'processing',
		Default = 'default',
		Checked = 'checked',
		Crossed = 'crossed',
		Archived = 'archived'
	}
</script>

<script lang="ts">
	import IconCheck from '~icons/mdi/check-circle-outline';
	import IconCircleOutline from '~icons/mdi/checkbox-blank-circle-outline';
	import IconOff from '~icons/mdi/circle-off-outline';
	import IconPillar from '~icons/mdi/pillar';
	import IconSync from '~icons/mdi/sync';

	export let state: State = State.Default;

	const animationClasses = 'cursor-pointer hover:scale-150 transform transition duration-200';
	let icon = IconCircleOutline;
	let color = 'text-blue-300';
	let spinning = false;
	$: {
		switch (state) {
			case State.Default:
				icon = IconCircleOutline;
				color = 'text-blue-300';
				spinning = false;
				break;
			case State.Checked:
				icon = IconCheck;
				color = 'text-success-600';
				spinning = false;
				break;
			case State.Crossed:
				icon = IconOff;
				color = 'text-error-600';
				spinning = false;
				break;
			case State.Archived:
				icon = IconPillar;
				color = 'text-gray-400';
				spinning = false;
				break;
			case State.Processing:
				icon = IconSync;
				color = 'text-secondary-300';
				spinning = true;
				break;
		}
	}
	$: klass = color + ' ' + (spinning ? 'animate-spin animate-reversed' : animationClasses);
	$: isChecked = state !== State.Default && state !== State.Processing;
</script>

<span class="text-2xl" role="checkbox" aria-checked={isChecked} on:click on:keypress tabindex="0">
	<svelte:component this={icon} class={klass} />
</span>

<style>
	:global(.animate-reversed) {
		animation-direction: reverse !important;
	}
</style>
