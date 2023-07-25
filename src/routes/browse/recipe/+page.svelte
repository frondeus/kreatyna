<script lang="ts">
	import { modalStore } from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import IconAdd from '~icons/mdi/add';
	import IconEdit from '~icons/mdi/edit';
	import IconDelete from '~icons/mdi/delete';
	import ConfirmDelete from '$lib/components/molecules/modals/ConfirmDelete.svelte';
	import { invalidateAll } from '$app/navigation';

	export let data: PageData;
	let { recipies } = data;
	$: ({ recipies } = data);

	const openDeleteModal = (event: MouseEvent, row: { id: number; name: string }) => {
		event.preventDefault();
		event.stopPropagation();
		modalStore.trigger({
			type: 'component',
			component: {
				ref: ConfirmDelete,
				props: {
					resource: 'recipe',
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
	<table class="table max-md:table-compact">
		<thead>
			<tr>
				<th>Name</th>
				<th class="flex justify-end">
					<a class="btn-icon variant-soft" href="./recipe/create">
						<IconAdd />
					</a>
				</th>
			</tr>
		</thead>
		<tbody>
			{#if !recipies}
				...Loading
			{:else}
				{#each recipies as row (row.id)}
					<tr>
						<td>
							<a href="./recipe/{row.id}">{row.name}</a>
						</td>
						<td class="flex justify-end gap-2">
							<a class="btn-icon variant-soft" href="./recipe/{row.id}">
								<IconEdit />
							</a>
							<button class="btn-icon variant-soft-error" on:click={(e) => openDeleteModal(e, row)}>
								<IconDelete />
							</button>
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
