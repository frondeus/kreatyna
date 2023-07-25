<script lang="ts">
	import { formFieldProxy } from 'sveltekit-superforms/client';
	import WithShim from '../forms-look/WithShim.svelte';

	export let form;
	export let title: string;
	export let name: string;

	const { path, value, errors, constraints } = formFieldProxy(form, name);
</script>

{#if $$slots.shim}
	<WithShim>
		<input
			class="input"
			{title}
			name={path}
			type="text"
			placeholder={title}
			bind:value={$value}
			aria-invalid={$errors ? 'true' : undefined}
			{...$constraints}
		/>
		<slot name="shim" />
	</WithShim>
{:else}
	<input
		class="input"
		{title}
		name={path}
		type="text"
		placeholder={title}
		bind:value={$value}
		aria-invalid={$errors ? 'true' : undefined}
		{...$constraints}
	/>
{/if}

{#if $errors} <small class="text-red-500"> {$errors} </small> {/if}
