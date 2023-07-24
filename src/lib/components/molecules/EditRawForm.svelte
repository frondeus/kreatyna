<script lang="ts">
	import { superForm } from 'sveltekit-superforms/client';
	import TextInput from '$lib/components/atoms/forms/TextInput.svelte';
	import NumberInput from '$lib/components/atoms/forms/NumberInput.svelte';
	import Submit from '$lib/components/atoms/forms/Submit.svelte';
	import type { AddRawSchema, EditRawSchema } from '$lib/schemas';
	import type { SuperValidated } from 'sveltekit-superforms';
	import { toastStore } from '@skeletonlabs/skeleton';

	export let id: number | undefined = undefined;
	export let form: SuperValidated<AddRawSchema | EditRawSchema>;

	const parentForm = superForm(form, {
		multipleSubmits: 'prevent',
		onUpdated: () => {
			toastStore.trigger({
				message: 'Raw Ingredient saved',
				background: 'variant-filled-success'
			});
		}
	});
	const { enhance, delayed } = parentForm;

	const title = id ? 'Edit Raw Ingredient' : 'Add Raw Ingredient';
	const submitDelayed = id ? 'Updating...' : 'Adding...';
	const submit = id ? 'Update' : 'Add';
</script>

<form class="card p-4 shadow-xl space-y-4" method="post" use:enhance>
	<h2 class="h2">{title}</h2>

	{#if id}
		<input type="hidden" name="id" value={id} />
	{/if}

	<TextInput form={parentForm} title="Name" name="name" />

	<NumberInput form={parentForm} title="Container" name="container" />

	<div class="grid grid-cols-[1fr_1fr_1fr] gap-4">
		<NumberInput form={parentForm} step="0.01" title="Proteins" name="proteins" />
		<NumberInput form={parentForm} step="0.01" title="Carbs" name="carbs" />
		<NumberInput form={parentForm} step="0.01" title="Fat" name="fat" />
	</div>

	<Submit value={$delayed ? submitDelayed : submit} />
</form>
