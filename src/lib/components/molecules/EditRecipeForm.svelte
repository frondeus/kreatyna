<script lang="ts">
	import { defaultValues, superForm } from 'sveltekit-superforms/client';
	import TextInput from '$lib/components/atoms/forms/TextInput.svelte';
	import NumberInput from '$lib/components/atoms/forms/NumberInput.svelte';
	import Submit from '$lib/components/atoms/forms/Submit.svelte';
	import { editIngredientSchema, type AddRecipeSchema, type EditRecipeSchema } from '$lib/schemas';
	import type { SuperValidated } from 'sveltekit-superforms';
	import IconAdd from '~icons/mdi/add';
	import IconRemove from '~icons/mdi/remove';
	import IngredientForm from './IngredientForm.svelte';
	import Textarea from '../atoms/forms/Textarea.svelte';
	import { toastStore } from '@skeletonlabs/skeleton';

	export let id: number | undefined = undefined;
	export let form: SuperValidated<AddRecipeSchema | EditRecipeSchema>;

	const parentForm = superForm(form, {
		multipleSubmits: 'prevent',
		dataType: 'json',
		onUpdated: () => {
			toastStore.trigger({
				message: 'Recipe saved',
				background: 'variant-filled-success'
			});
		}
	});
	const { enhance, delayed, form: _form } = parentForm;

	const title = id ? 'Edit Recipe' : 'Add Recipe';
	const submitDelayed = id ? 'Updating...' : 'Adding...';
	const submit = id ? 'Update' : 'Add';

	const addStep = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		$_form.steps = [...$_form.steps, ''];
	};
	const removeStep = (event: MouseEvent, idx: number) => {
		event.preventDefault();
		event.stopPropagation();
		$_form.steps.splice(idx, 1);
		$_form.steps = $_form.steps;
	};
	const addIngredient = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		const def = defaultValues(editIngredientSchema);
		console.log({ def });
		$_form.ingredients = [...$_form.ingredients, def];
	};
	const removeIngredient = (event: CustomEvent<number>) => {
		const idx = event.detail;
		$_form.ingredients.splice(idx, 1);
		$_form.ingredients = $_form.ingredients;
	};
</script>

<form class="card lg:p-4 shadow-xl space-y-4" method="post" use:enhance>
	<div class="p-2">
		<h2 class="h2 mb-4">{title}</h2>

		{#if id}
			<input type="hidden" name="id" value={id} />
		{/if}

		<div class="mb-2"><TextInput form={parentForm} title="Name" name="name" /></div>

		<NumberInput form={parentForm} title="Portions" name="portions" />
	</div>
	<div class="grid lg:grid-cols-2 gap-2">
		<div class="card lg:p-4">
			<div class="table-container">
				<table class="table max-md:table-compact">
					<thead>
						<tr>
							<th>Ingredient</th>
							<th>Amount</th>
							<th>Unit</th>
							<th class="table-cell-fit">
								<button class="btn-icon variant-soft" on:click={addIngredient}>
									<IconAdd />
								</button>
							</th>
						</tr>
					</thead>
					<tbody>
						{#each $_form.ingredients as _, i (i)}
							<IngredientForm form={parentForm} idx={i} on:remove={removeIngredient} />
						{/each}
					</tbody>
				</table>
			</div>
		</div>
		<div class="card p-2 lg:p-4">
			<h2 class="h2 mb-4">Steps</h2>
			<ol class="list space-y-4">
				{#each $_form.steps as _, i (i)}
					<li>
						<Textarea form={parentForm} title="Step" name="steps" field={`steps[${i}]`} />
						<button class="btn-icon variant-soft" on:click={(e) => removeStep(e, i)}>
							<IconRemove />
						</button>
					</li>
				{/each}
				<li>
					<button class="input btn variant-soft" on:click={addStep}> Add step </button>
				</li>
			</ol>
		</div>
	</div>

	<div class="p-2">
		<Submit value={$delayed ? submitDelayed : submit} />
	</div>
</form>
