<!-- <script lang="ts">
	import { defaultValues, superForm, superValidateSync } from "sveltekit-superforms/client";
  import {  editIngredientSchema, editRecipeSchema } from "./schema";
	import { modalStore } from "@skeletonlabs/skeleton";
	import IconAdd from '~icons/mdi/add';
	import EditIngredientForm from "./EditIngredientForm.svelte";
	import type { Recipy } from "./types";

	export let parent: {
		onClose: () => void;
	};

  export let row: Recipy;


	$: parentForm = superForm(
    superValidateSync(row, editRecipeSchema)
, {
		multipleSubmits: 'prevent',
		dataType: 'json',
		resetForm: true,
		onResult: ({ result }) => {
			if(result.type === "success") {
				parent.onClose();
			}
		}
	});


	$: ({ form, errors, constraints, enhance, delayed, message } = parentForm);

  $: { console.log({ row, parentForm, form, $form, editRecipeSchema , $message}); }

	const addStep = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		$form.steps = [...$form.steps, ''];
	};
  const addIngredient = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		const def =  defaultValues(editIngredientSchema);
		$form.ingredients = [...$form.ingredients, def];
	};

	const removeIngredient = (event: CustomEvent<number>) => {
		const idx = event.detail;
		$form.ingredients.splice(idx, 1);
		$form.ingredients = $form.ingredients;
	};
</script>

{#if $modalStore[0]}
	<form
		class="form-widget card p-4 w-modal-wide shadow-xl space-y-4"
		method="POST"
		action="?/editRecipe"
		use:enhance
	>
		<h2 class="h2">Edit Recipe</h2>
    <input
      name="id"
      type="hidden"
      value={$form.id}
    />
		<input
			class="input"
			title="Name"
			name="name"
			type="text"
			placeholder="Name"
			bind:value={$form.name}
			aria-invalid={$errors.name ? 'true' : undefined}
			{...$constraints.name}
		/>
    {#if $errors.name}<small class="text-red-500">{$errors.name}</small>{/if}

		<label class="label">
			<span>Portions</span>
  		<input
  			class="input"
  			title="Portions"
  			name="portions"
  			type="number"
  			bind:value={$form.portions}
  			aria-invalid={$errors.portions ? 'true' : undefined}
        class:input-error={$errors.portions}
				{...$constraints.portions}
  		/>
      </label>
    {#if $errors.portions}<small class="text-red-500">{$errors.portions}</small>{/if}

    <div class="grid grid-cols-4 gap-4">
      <div class="card p-4 col-span-2">
        <h4 class="h4">Steps</h4>

					<ol class="list">
					{#each $form.steps as _, i (i)}
					<li>
							<span>{i+1}</span>
							<textarea class="textarea"
								rows="2"
								placeholder="Enter next step"
								name="steps"
								bind:value={$form.steps[i]}
				  			aria-invalid={$errors.steps?.[i] ? 'true' : undefined}
				        class:input-error={$errors.steps?.[i]}
								{...$constraints.steps}
							/>
					</li>
					{/each}
					<li>
						<span>{$form.steps.length + 1}</span>
					<button class="btn variant-soft" on:click={addStep}>
							Add step
					</button>
						</li>
					</ol>

      </div>
      <div class="card p-4 col-span-2">
        <h4 class="h4">Ingredients</h4>
        <div class="table-container">
          <table class="table">
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
							{#each $form.ingredients as _, i (i)}
								<EditIngredientForm form={parentForm} idx={i} on:remove={removeIngredient} />
							{/each}
						</tbody>
          </table>
        </div>
      </div>
    </div>
    
		<input type="submit" class="input btn variant-filled" value={$delayed ? 'Updating...' : 'Update'} />
	</form>
{/if} -->
