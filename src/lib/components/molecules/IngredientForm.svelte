<script lang="ts">
	import type { AddRecipeSchema, EditRecipeSchema } from '$lib/schemas';
	import { createEventDispatcher } from 'svelte';
	import IconRemove from '~icons/mdi/remove';
	import IconLink from '~icons/mdi/link-variant-plus';
	import IconUnlink from '~icons/mdi/link-variant-off';
	import TextInput from '../atoms/forms/TextInput.svelte';
	import NumberInput from '../atoms/forms/NumberInput.svelte';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import HiddenInput from '../atoms/forms/HiddenInput.svelte';
	import { modalStore } from '@skeletonlabs/skeleton';
	import SearchForRawIngredient from '../molecules/modals/SearchForRawIngredient.svelte';
	import type { RawIngredient } from '$lib/types';
	import WithShim from '../atoms/forms-look/WithShim.svelte';

	export let relatedIngredients: Map<number, RawIngredient>;
	export let form: SuperForm<AddRecipeSchema | EditRecipeSchema>;
	export let idx: number;
	//   export let id: number | undefined = undefined;

	const { form: _form } = form;
	const dispatcher = createEventDispatcher();

	let relatedIngredient: RawIngredient | undefined;

	$: {
		const currentIngredient = $_form.ingredients[idx];
		const rawId = currentIngredient.rawId;
		relatedIngredient = rawId ? relatedIngredients.get(rawId) : undefined;
	}

	const onRemove = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		dispatcher('remove', idx);
	};

	const onLink = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();

		modalStore.trigger({
			type: 'component',
			component: {
				ref: SearchForRawIngredient,
				props: {
					name: $_form.ingredients[idx].name
				}
			},
			response: (newRaw: RawIngredient) => {
				if (newRaw === undefined) return;
				relatedIngredients.set(newRaw.id, newRaw);
				$_form.ingredients[idx].rawId = newRaw.id;
			}
		});
	};

	const onUnlink = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();

		$_form.ingredients[idx].rawId = undefined;
	};
</script>

<tr>
	<td>
		{#if $_form.ingredients[idx].id}
			<HiddenInput {form} field={`ingredients[${idx}].id`} />
		{/if}

		{#if relatedIngredient}
			<HiddenInput {form} field={`ingredients[${idx}].rawId`} />
			<HiddenInput {form} field={`ingredients[${idx}].name`} />

			<WithShim>
				<input class="input" type="text" disabled value={relatedIngredient?.name ?? ''} />
				<button title="Unlink ingredient" class="variant-filled-surface" on:click={onUnlink}>
					<IconUnlink />
				</button>
			</WithShim>
		{:else}
			<TextInput {form} title="Name" name={`ingredients[${idx}].name`}>
				<svelte:fragment slot="shim">
					<button title="Link to raw ingredient" class="variant-filled-primary" on:click={onLink}>
						<IconLink />
					</button>
				</svelte:fragment>
			</TextInput>
		{/if}
	</td>
	<td>
		<NumberInput {form} step="0.1" title="Amount" nolabel name={`ingredients[${idx}].amount`} />
	</td>
	<td>
		<TextInput {form} title="Unit" name={`ingredients[${idx}].unit`} />
	</td>

	<td>
		<button title="Remove" class="btn-icon variant-soft" on:click={onRemove}>
			<IconRemove />
		</button>
	</td>
</tr>
