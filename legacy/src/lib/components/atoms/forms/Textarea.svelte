<script lang="ts">
	import { formFieldProxy } from 'sveltekit-superforms/client';

	export let form;
	export let field: string;
	export let title: string;
	export let name: string | undefined = undefined;

	const { path, value, errors, constraints } = formFieldProxy(form, field);
</script>

<textarea
	class="textarea"
	rows="2"
	{title}
	name={name ?? path}
	placeholder={title}
	bind:value={$value}
	aria-invalid={$errors ? 'true' : undefined}
	{...$constraints}
/>

{#if $errors} <small class="text-red-500"> {$errors} </small> {/if}
