<script lang="ts">
	import IconAdd from '~icons/mdi/add';
	import IconEdit from '~icons/mdi/edit';
	import IconDelete from '~icons/mdi/delete';
	import type { PageData } from './$types';
	import { modalStore } from '@skeletonlabs/skeleton';
	import ConfirmDelete from '$lib/components/molecules/modals/ConfirmDelete.svelte';
	import { readable } from 'svelte/store';
	import { invalidateAll } from '$app/navigation';
	import type { RawIngredient } from '$lib/shared/domains/raw/models';

	export let data: PageData;

	let rawIngredients = readable(data.rawIngredients ?? []);

	const openDeleteModal = (event: MouseEvent, row: RawIngredient) => {
		event.preventDefault();
		event.stopPropagation();
		modalStore.trigger({
			type: 'component',
			component: {
				ref: ConfirmDelete,
				props: {
					resource: 'raw ingredient',
					id: row.id,
					name: row.name
				}
			},
			response: () => {
				invalidateAll();
			}
		});
	};
</script>

<div class="table-container">
	<table class="table-hover table max-md:table-compact">
		<thead>
			<tr>
				<th>Name</th>
				<th class="max-md:hidden">Container</th>
				<th class="max-md:hidden">Proteins</th>
				<th class="max-md:hidden">Carbs</th>
				<th class="max-md:hidden">Fat</th>
				<th class="flex justify-end">
					<a class="btn-icon variant-soft" href={'./raw/create'}>
						<IconAdd />
					</a>
				</th>
			</tr>
		</thead>
		<tbody>
			{#if !$rawIngredients}
				...Loading
			{:else}
				{#each $rawIngredients as row (row.id)}
					<tr>
						<td> {row.name} </td>
						<td class="max-md:hidden"> {row.container} </td>
						<td class="max-md:hidden">
							<span class="badge variant-ghost"> {row.proteins} </span>
						</td>
						<td class="max-md:hidden"> <span class="badge variant-ghost"> {row.carbs} </span> </td>
						<td class="max-md:hidden"> <span class="badge variant-ghost"> {row.fat} </span> </td>
						<td class="flex justify-end gap-2">
							{#if row.createdByMe}
								<a class="btn-icon variant-soft" href="./raw/{row.id}">
									<IconEdit />
								</a>
								<button
									class="btn-icon variant-soft-error"
									on:click={(e) => openDeleteModal(e, row)}
								>
									<IconDelete />
								</button>
							{/if}
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
