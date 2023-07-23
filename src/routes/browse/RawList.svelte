<script lang="ts">
	import IconAdd from '~icons/mdi/add';
	import IconEdit from '~icons/mdi/edit';
	import IconDelete from '~icons/mdi/delete';
	import { modalStore, type ModalComponent } from '@skeletonlabs/skeleton';
	import AddRawModal from './AddRawModal.svelte';
	import EditRawModal from './EditRawModal.svelte';
	import { superForm } from 'sveltekit-superforms/client';
	import type { PageData } from './$types';
	import { fromDB, type RawIngredient, type RawIngredientRow } from './types';
	import { realtime } from '$lib/stores';

	export let data: PageData;
	let { supabase, session } = data;
	$: ({ supabase, session } = data);

	const rawIngredients = realtime(
		supabase,
		data.rawIngredients ?? [],
		'raw',
		(row: RawIngredientRow) => fromDB.raw(session, row)
	);

	const addRawModalOpen = () => {
		const modalComponent: ModalComponent = {
			ref: AddRawModal,
			props: {
				addRawForm: data.addRawForm
			}
		};
		modalStore.trigger({
			type: 'component',
			component: modalComponent
		});
	};
	const editRawModalOpen = (row: RawIngredient) => {
		const modalComponent: ModalComponent = {
			ref: EditRawModal,
			props: {
				row
			}
		};
		modalStore.trigger({
			type: 'component',
			component: modalComponent
		});
	};
	const { enhance } = superForm(data.deleteRawForm);
</script>

<div class="md:table-container">
	<table class="table">
		<thead>
			<tr>
				<th>Name</th>
				<th>Container</th>
				<th>Proteins</th>
				<th>Carbs</th>
				<th>Fat</th>
				<th>
					<button class="btn-icon variant-soft" on:click={addRawModalOpen}>
						<IconAdd />
					</button>
				</th>
			</tr>
		</thead>
		<tbody>
			{#if !$rawIngredients}
				...Loading
			{:else}
				{#each $rawIngredients as row (row.name)}
					<tr>
						<td> {row.name} </td>
						<td> {row.container} </td>
						<td> <span class="badge variant-ghost"> {row.proteins} </span> </td>
						<td> <span class="badge variant-ghost"> {row.carbs} </span> </td>
						<td> <span class="badge variant-ghost"> {row.fat} </span> </td>
						<td>
							{#if row.createdByMe}
								<button class="btn-icon variant-soft" on:click={() => editRawModalOpen(row)}>
									<IconEdit />
								</button>
								<form class="inline" method="post" action="?/deleteRaw" use:enhance>
									<input type="hidden" name="id" value={row.id} />
									<button class="input btn-icon variant-filled-error">
										<IconDelete />
									</button>
								</form>
							{/if}
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
