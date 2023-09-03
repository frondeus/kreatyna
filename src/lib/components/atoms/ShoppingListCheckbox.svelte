<script context="module" lang="ts">
	export enum State {
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

	export let state: State = State.Default;

	const animationClasses = 'cursor-pointer hover:scale-150 transform transition duration-200';
</script>

<span class="text-2xl">
	{#if state === State.Default}
		<IconCircleOutline class="text-blue-300 {animationClasses}" on:click on:keypress />
	{:else if state === State.Checked}
		<IconCheck class="text-success-600 {animationClasses}" on:click on:keypress />
	{:else if state === State.Crossed}
		<IconOff class="text-error-600 {animationClasses}" on:click on:keypress />
	{:else if state === State.Archived}
		<IconPillar class="text-gray-400 {animationClasses}" on:click on:keypress />
	{/if}
</span>
