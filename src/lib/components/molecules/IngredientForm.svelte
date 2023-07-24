<script lang="ts">
	import type { AddRecipeSchema, EditRecipeSchema } from '$lib/schemas';
	import { createEventDispatcher } from 'svelte';
	import IconRemove from '~icons/mdi/remove';
	import TextInput from '../atoms/forms/TextInput.svelte';
	import NumberInput from '../atoms/forms/NumberInput.svelte';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import HiddenInput from '../atoms/forms/HiddenInput.svelte';

	export let form: SuperForm<AddRecipeSchema | EditRecipeSchema>;
	export let idx: number;
	//   export let id: number | undefined = undefined;

	const { form: _form } = form;
	const dispatcher = createEventDispatcher();

	const onRemove = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		dispatcher('remove', idx);
	};
</script>

<tr>
	<td>
		{#if $_form.ingredients[idx].id}
			<HiddenInput {form} field={`ingredients[${idx}].id`} />
		{/if}
		<TextInput {form} title="Name" name={`ingredients[${idx}].name`} />
	</td>
	<td>
		<NumberInput {form} title="Amount" nolabel name={`ingredients[${idx}].amount`} />
	</td>
	<td>
		<TextInput {form} title="Unit" name={`ingredients[${idx}].unit`} />
	</td>

	<td>
		<button class="btn-icon variant-soft" on:click={onRemove}>
			<IconRemove />
		</button>
	</td>
</tr>
