<script lang="ts">
	import IconAdd from '~icons/mdi/add';
	import IconEdit from '~icons/mdi/edit';
	import IconDelete from '~icons/mdi/delete';
	import { modalStore, type ModalComponent } from '@skeletonlabs/skeleton';
	import AddRawModal from './AddRawModal.svelte';
	import EditRawModal from './EditRawModal.svelte';
	import { superForm } from 'sveltekit-superforms/client';
	import type { PageData } from './$types';

	interface RawIngredient {
		id: number;
		name: string;
		container: number;
		proteins: number;
		carbs: number;
		fat: number;
		createdByMe: boolean;
	}

	export let data: PageData;

	const addRawModalOpen = () => {
		const modalComponent: ModalComponent = {
			ref: AddRawModal,
			props: {
				data
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
				data,
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
			{#if !data.rawIngredients}
				...Loading
			{:else}
				{#each data.rawIngredients as row (row.name)}
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
