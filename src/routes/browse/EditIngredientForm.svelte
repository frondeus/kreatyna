
<script lang="ts">
	import { createEventDispatcher } from "svelte";
	import { formFieldProxy, type SuperForm } from "sveltekit-superforms/client";
	import IconRemove from '~icons/mdi/remove';
	import type { EditRecipeSchema } from "./schema";

  export let form: SuperForm<EditRecipeSchema>;
  export let idx: number;

  const { value: id } = formFieldProxy(form, `ingredients[${idx}].id`);
  const { value: name, errors: nameErrors, constraints: nameConstraints } = formFieldProxy(form, `ingredients[${idx}].name`);
  const { value: amount, errors: amountErrors, constraints: amountConstraints } = formFieldProxy(form, `ingredients[${idx}].amount`);
  const { value: unit, errors: unitErrors, constraints: unitConstraints } = formFieldProxy(form, `ingredients[${idx}].unit`);

  // const unit = stringProxy(form, `ingredients[${idx}].unit`, { empty: 'undefined' });
  const dispatcher = createEventDispatcher();

  const onRemove = (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation();
    dispatcher('remove', idx);  
  };
</script>

<tr>
	<td>
      <input type="hidden"
        value={$id}
      />
    
			<input type="text" class="input"
				title="Name"
				placeholder="Name"
				bind:value={$name}
				aria-invalid={$nameErrors ? 'true': undefined}
				class:input-error={$nameErrors}
				{...$nameConstraints}
				/>
	</td>
	<td>
			<input type="number" class="input"
				title="Amount"
				bind:value={$amount}
				aria-invalid={$amountErrors ? 'true': undefined}
				class:input-error={$amountErrors}
				{...$amountConstraints}
				/>
	</td>
	<td>
			<input type="text" class="input"
				title="Unit"
				placeholder="g"
				bind:value={$unit}
				aria-invalid={$unitErrors ? 'true': undefined}
				class:input-error={$unitErrors}
				{...$unitConstraints}
				/>
	</td>

	<td>
		<button class="btn-icon variant-soft" on:click={onRemove}>
			<IconRemove />
		</button>
	</td>
</tr>