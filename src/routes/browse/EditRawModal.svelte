<script lang="ts">
	import { modalStore } from '@skeletonlabs/skeleton';
	import { superForm, superValidateSync } from 'sveltekit-superforms/client';
	import { editRawSchema } from './schema';
	import type { RawIngredient } from './types';

	export let parent: {
		onClose: () => void;
	};

	export let row: RawIngredient;

	const { form, errors, constraints, enhance, delayed } = superForm(
		superValidateSync(row, editRawSchema),
		{
			multipleSubmits: 'prevent',
			onUpdated: ({ form }) => {
				if (form.valid) {
					parent.onClose();
				}
			}
		}
	);
</script>

{#if $modalStore[0]}
	<form
		class="form-widget card p-4 w-modal-wide shadow-xl space-y-4"
		method="POST"
		action="?/editRaw"
		use:enhance
	>
		<h2 class="h2">Edit Raw Ingredient</h2>
		<input class="input" type="hidden" name="id" value={$form.id} />

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

		{#if $errors.name}
			<span class="invalid">
				{$errors.name}
			</span>
		{/if}

		<label class="label">
			<span>Container</span>
			<div class="input-group input-group-divider grid-cols-[1fr_auto]">
				<input
					title="Container"
					name="container"
					type="number"
					bind:value={$form.container}
					aria-invalid={$errors.container ? 'true' : undefined}
					{...$constraints.container}
				/>
				<div class="input-group-shim">(g)</div>
			</div>
		</label>

		<div class="grid grid-cols-[1fr_1fr_1fr] gap-4">
			<label class="label">
				<span>Proteins</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input
						type="number"
						name="proteins"
						title="Proteins"
						step="0.01"
						bind:value={$form.proteins}
						aria-invalid={$errors.proteins ? 'true' : undefined}
						{...$constraints.proteins}
					/>
					<div class="input-group-shim">(g)</div>
				</div>
			</label>
			<label class="label">
				<span>Carbs</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input
						type="number"
						name="carbs"
						title="Carbs"
						step="0.01"
						bind:value={$form.carbs}
						aria-invalid={$errors.carbs ? 'true' : undefined}
						{...$constraints.carbs}
					/>
					<div class="input-group-shim">(g)</div>
				</div>
			</label>
			<label class="label">
				<span>Fat</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input
						type="number"
						name="fat"
						title="Fat"
						step="0.01"
						bind:value={$form.fat}
						aria-invalid={$errors.fat ? 'true' : undefined}
						{...$constraints.fat}
					/>
					<div class="input-group-shim">(g)</div>
				</div>
			</label>
		</div>

		<input
			type="submit"
			class="input btn variant-filled"
			value={$delayed ? 'Updating...' : 'Update'}
		/>
	</form>
{/if}
