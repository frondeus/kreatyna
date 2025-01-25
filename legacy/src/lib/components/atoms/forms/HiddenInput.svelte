<script lang="ts">
	import { formFieldProxy } from 'sveltekit-superforms/client';

	export let form;
	export let field: string;
	export let name: string | undefined = undefined;

	const { path, value, errors, constraints } = formFieldProxy(form, field);
</script>

<input
	class="input"
	name={name ?? path}
	type="hidden"
	bind:value={$value}
	aria-invalid={$errors ? 'true' : undefined}
	{...$constraints}
/>
