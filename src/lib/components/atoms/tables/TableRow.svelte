<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { modalStore } from '@skeletonlabs/skeleton';
	import IconEdit from '~icons/mdi/edit';
	import IconDelete from '~icons/mdi/delete';
	import ConfirmDelete from '$components/organisms/modals/ConfirmDelete.svelte';
	import type { ComponentType, SvelteComponent } from 'svelte';

	export let id: string | number;
	export let href: string;

	export let name: string;

    export let extra: ComponentType<SvelteComponent>[] = [];

	export let editable = false;
	export let deletable = false;

	const openDeleteModal = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();
		modalStore.trigger({
			type: 'component',
			component: {
				ref: ConfirmDelete,
				props: {
					resource: 'recipe',
					id,
					name
				}
			},
			response: () => {
				invalidateAll();
			}
		});
	};
</script>

<tr>
	<td>
		<a href="{href}/{id}">{name}</a>
	</td>
	{#each extra as Extra}
		<td>
			<svelte:component this={Extra} />
		</td>
	{/each}
	<td class="flex justify-end gap-2">
		{#if editable}
			<a class="btn-icon variant-soft" href="{href}/{id}">
				<IconEdit />
			</a>
		{/if}
		{#if deletable}
			<button class="btn-icon variant-soft-error" on:click={(e) => openDeleteModal(e)}>
				<IconDelete />
			</button>
		{/if}
	</td>
</tr>
