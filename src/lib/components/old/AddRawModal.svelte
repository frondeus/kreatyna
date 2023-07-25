<!-- <script lang="ts">
	import { modalStore } from '@skeletonlabs/skeleton';
	import { superForm } from 'sveltekit-superforms/client';

	export let parent: {
		onClose: () => void;
	};

	export let addRawForm;

	const { form, errors, constraints, enhance, delayed } = superForm(addRawForm, {
		multipleSubmits: 'prevent',
		resetForm: true,
		onResult: ({ result }) => {
			if (result.type === "success") {
				parent.onClose();
			}
		}
	});
</script>

{#if $modalStore[0]}
	<form
		class="form-widget card p-4 w-modal-wide shadow-xl space-y-4"
		method="POST"
		action="?/addRaw"
		use:enhance
	>
		<h2 class="h2">Add Raw Ingredient</h2>
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

		{#if $errors.name} <small class="text-red-500"> {$errors.name} </small> {/if}

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
		{#if $errors.container} <small class="text-red-500"> {$errors.container} </small> {/if}

		<div class="grid grid-cols-[1fr_1fr_1fr] gap-4">
			<label class="label">
				<span>Proteins</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input
						type="number"
						step="0.01"
						name="proteins"
						title="Proteins"
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
						step="0.01"
						title="Carbs"
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
		{#if $errors.proteins} <small class="text-red-500"> {$errors.proteins} </small> {/if}
		{#if $errors.carbs} <small class="text-red-500"> {$errors.carbs} </small> {/if}
		{#if $errors.fat} <small class="text-red-500"> {$errors.fat} </small> {/if}

		<input type="submit" class="input btn variant-filled" value={$delayed ? 'Adding...' : 'Add'} />
	</form>
{/if} -->
