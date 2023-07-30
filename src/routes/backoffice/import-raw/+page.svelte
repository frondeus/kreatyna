<script>
	import { enhance } from '$app/forms';
	import Submit from '$lib/components/atoms/forms/Submit.svelte';

	export let form;

	$: console.log(form?.error);
</script>

<h1 class="h1">Import Raw Ingredients</h1>

{#if form?.error}
	<h2 class="h2">{form?.error}</h2>
{/if}

<form class="card p-4 shadow-xl space-y-4" method="post" use:enhance>
	<label class="label">
		<span>XLSX File</span>
		<input class="input" type="file" name="xlsx" accept=".xlsx" required />
	</label>

	<Submit value="Import" />
</form>
