<script lang="ts">
	import { formFieldProxy } from 'sveltekit-superforms/client';
	import WithShim from '../forms-look/WithShim.svelte';

	export let form;
	export let title: string;
	export let name: string;
	export let nolabel = false;
	export let step: string | undefined = undefined;

	const { path, value, errors, constraints } = formFieldProxy(form, name);
</script>

<label class="label">
	{#if !nolabel}
		<span>{title}</span>
	{/if}

	{#if $$slots.shim}
		<WithShim>
			<input
				class="input"
				{title}
				{step}
				name={path}
				type="number"
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
			{step}
			name={path}
			type="number"
			placeholder={title}
			bind:value={$value}
			aria-invalid={$errors ? 'true' : undefined}
			{...$constraints}
		/>
	{/if}
</label>

{#if $errors} <small class="text-red-500"> {$errors} </small> {/if}
