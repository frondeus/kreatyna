<script lang="ts">
	import { defaultValues, superForm } from 'sveltekit-superforms/client';
	import TextInput from '$lib/components/atoms/forms/TextInput.svelte';
	import NumberInput from '$lib/components/atoms/forms/NumberInput.svelte';
	import Submit from '$lib/components/atoms/forms/Submit.svelte';
	import type { SuperValidated } from 'sveltekit-superforms';
	import IconAdd from '~icons/mdi/add';
	import IconRemove from '~icons/mdi/remove';
	import IngredientForm from '../molecules/IngredientForm.svelte';
	import Textarea from '../atoms/forms/Textarea.svelte';
	import { toastStore, type PopupSettings, popup } from '@skeletonlabs/skeleton';
	import { Calculator } from '$lib/shared/calc';
	import Checkbox from '../atoms/forms/Checkbox.svelte';
	import type { RawIngredient } from '$lib/shared/domains/raw/models';
	import {
		editIngredientSchema,
		type AddOrEditRecipeSchema
	} from '$lib/shared/domains/recipe/schema';
	import Tag from '$components/atoms/Tag.svelte';
	import SearchForTag from './SearchForTag.svelte';
	import type { TagWithCategory } from '$shared/domains/tags/models';

	export let form: SuperValidated<AddOrEditRecipeSchema>;
	export let relatedIngredients: Map<number, RawIngredient>;

	const parentForm = superForm(form, {
		multipleSubmits: 'prevent',
		dataType: 'json',
		onError: ({ result }) => {
			toastStore.trigger({
				message: result.error.message,
				background: 'variant-filled-error'
			});
		},
		onUpdated: ({ form }) => {
			if (form.valid) {
				toastStore.trigger({
					message: 'Recipe saved',
					background: 'variant-filled-success'
				});
			}
		}
	});
	const { enhance, delayed, form: _form } = parentForm;

	$: id = $_form.id;
	$: title = id ? 'Edit Recipe' : 'Add Recipe';
	$: submitDelayed = $_form.askGPT ? 'Asking...' : id ? 'Updating...' : 'Adding...';
	$: submit = $_form.askGPT ? 'Ask' : id ? 'Update' : 'Add';
	const addTagPopup: PopupSettings = {
		event: 'click',
		target: 'tagPopupClick',
		placement: 'bottom-start'
	};

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
		$_form.ingredients = [...$_form.ingredients, def];
	};
	const removeIngredient = (event: CustomEvent<number>) => {
		const idx = event.detail;
		$_form.ingredients.splice(idx, 1);
		$_form.ingredients = $_form.ingredients;
	};
	const addTag = (event: CustomEvent<TagWithCategory>) => {
		$_form.tags = [
			...$_form.tags,
			{ id: event.detail.id, name: event.detail.name, category: event.detail.category.name }
		];
	};

	$: nutrients = Calculator.calcNutrientsForIngredients({
		ingredients: $_form.ingredients.map((ingredient) => ({
			...ingredient,
			raw: ingredient.rawId ? relatedIngredients.get(ingredient.rawId) : undefined
		}))
	});
</script>

<form class="card lg:p-4 shadow-xl space-y-4" method="post" use:enhance>
	<div class="p-2 space-y-4">
		<h2 class="h2">{title}</h2>

		{#if id}
			<input type="hidden" name="id" bind:value={$_form.id} />
		{/if}

		<div class="mb-2"><TextInput form={parentForm} title="Name" field="name" /></div>

		<NumberInput form={parentForm} title="Portions" name="portions" />

		<div class="flex gap-2">
			{#each $_form.tags as _, i (i)}
				<Tag form={parentForm} idx={i} />
			{/each}

			<span class="badge variant-filled-primary cursor-pointer" use:popup={addTagPopup}>
				<IconAdd /> Add tag
			</span>
		</div>

		<div class="card p-4 variant-filled-surface z-10" data-popup="tagPopupClick">
			<SearchForTag on:add={addTag} />
		</div>
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
							<IngredientForm
								form={parentForm}
								idx={i}
								on:remove={removeIngredient}
								{relatedIngredients}
							/>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="table-container mt-4">
				<table class="table max-md:table-compact">
					<thead>
						<tr>
							<th>Nutrient</th>
							<th>Amount</th>
							<th>Unit</th>
						</tr>
					</thead>
					<tbody>
						<tr>
							<td>Energy</td>
							<td>{Calculator.calcKcal(nutrients)}</td>
							<td>kcal</td>
						</tr>
						<tr>
							<td>Protein</td>
							<td>{nutrients.proteins}</td>
							<td>g</td>
						</tr>
						<tr>
							<td>Carbohydrates</td>
							<td>{nutrients.carbs}</td>
							<td>g</td>
						</tr>
						<tr>
							<td>Fat</td>
							<td>{nutrients.fat}</td>
							<td>g</td>
						</tr>
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

	<div class="p-2 space-y-4">
		<Checkbox form={parentForm} title="Ask GPT to fill the blanks" name="askGPT" />
		<Submit value={$delayed ? submitDelayed : submit} />
	</div>
</form>
