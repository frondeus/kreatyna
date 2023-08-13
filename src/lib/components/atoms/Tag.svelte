<script lang="ts">
	import type { AddOrEditRecipeSchema } from '$shared/domains/recipe/schema';
	import { formFieldProxy, type SuperForm } from 'sveltekit-superforms/client';
	import HiddenInput from './forms/HiddenInput.svelte';

	export let form: SuperForm<AddOrEditRecipeSchema>;
	export let idx: number;

	const _form = form.form;

	const onRemove = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		$_form.tags.splice(idx, 1);
		$_form.tags = $_form.tags;
	};

	const { value } = formFieldProxy(form, `tags[${idx}].name`);
	const { value: cat } = formFieldProxy(form, `tags[${idx}].category`);
</script>

<span class="badge variant-filled">
	<HiddenInput {form} field={`tags[${idx}].id`} />
	{$cat}: {$value}
	<button on:click={onRemove}> x </button>
</span>
