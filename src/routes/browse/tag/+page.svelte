<script lang="ts">
	import IconAdd from '~icons/mdi/add';
	import IconDelete from '~icons/mdi/delete';
	import { modalStore } from '@skeletonlabs/skeleton';
	import { invalidateAll } from '$app/navigation';
	import type { TagWithCategory } from '$shared/domains/tags/models';
	import ConfirmDelete from '$components/organisms/modals/ConfirmDelete.svelte';
	import { readable } from 'svelte/store';
	import type { PageData } from './$types';

	export let data: PageData;
	const tags = readable(data.tags ?? []);

	const openDeleteModal = (event: MouseEvent, row: TagWithCategory) => {
		event.preventDefault();
		event.stopPropagation();
		modalStore.trigger({
			type: 'component',
			component: {
				ref: ConfirmDelete,
				props: {
					resource: 'tag',
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
				<th class="flex justify-end">
					<a class="btn-icon variant-soft" href={'./tag/create'}>
						<IconAdd />
					</a>
				</th>
			</tr>
		</thead>
		<tbody>
			{#if !$tags}
				...Loading
			{:else}
				{#each $tags as row (row.id)}
					<tr>
						<td>
							<span class="badge variant-filled">
								{row.category.name + ':' + row.name}
							</span>
						</td>
						<td class="flex justify-end gap-2">
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
