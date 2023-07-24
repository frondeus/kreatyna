<script lang="ts">
	import { formFieldProxy } from 'sveltekit-superforms/client';

	export let form;
	export let title: string;
	export let name: string;

	const { path, value, errors, constraints } = formFieldProxy(form, name);
</script>

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

{#if $errors} <small class="text-red-500"> {$errors} </small> {/if}
