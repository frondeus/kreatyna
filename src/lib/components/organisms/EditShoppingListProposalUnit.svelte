<script lang="ts">
	import WithShim from '$components/atoms/forms-look/WithShim.svelte';
	import { drawerStore } from '@skeletonlabs/skeleton';
	import IconCross from '~icons/mdi/close-thick';

	export let name: string;
	export let unit: string | undefined = undefined;
	export let amount: number;
	export let onSubmit: (amount: number, unit: string | undefined) => void;

	$: onSubmit(amount, unit);

	const submit = () => {
		drawerStore.close();
	};

	const addUnit = () => {
		unit = 'g';
	};
	const removeUnit = () => {
		unit = undefined;
	};
</script>

<div class="space-y-4">
	<h2 class="h2">Edit {name}</h2>

	<div class="grid grid-cols-5 gap-4">
		<label class="label col-span-3">
			<span>Amount</span>
			<input class="input" type="number" bind:value={amount} />
		</label>
		{#if unit !== undefined}
			<label class="label col-span-2">
				<span>Unit</span>
				<WithShim>
					<input class="input" type="text" bind:value={unit} />
					<button class="variant-filled-error" on:click={removeUnit} on:keypress={removeUnit}>
						<IconCross class="" />
					</button>
				</WithShim>
			</label>
		{:else}
			<button
				class="btn input variant-filled-surface margin-23 col-span-2"
				on:click={addUnit}
				on:keypress={addUnit}
			>
				Add unit
			</button>
		{/if}
	</div>

	<button class="btn input variant-filled-primary" on:click={submit} on:keypress={submit}
		>Save</button
	>
</div>

<style>
	.margin-23 {
		margin-top: 25px;
	}
</style>
