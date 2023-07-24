<script lang="ts">
	import { modalStore, type ModalComponent } from "@skeletonlabs/skeleton";
	import type { PageData } from "./$types";
  import AddRecipeModal from "./AddRecipeModal.svelte";
  import EditRecipeModal from "./EditRecipeModal.svelte";
	import IconAdd from '~icons/mdi/add';
	import type { Recipy } from "./types";


  export let data: PageData;
  let { supabase, session, recipies } = data;
  $: ({ supabase, session, recipies } = data);

  const addRecipeModalOpen = () => {
    const component: ModalComponent = {
      ref: AddRecipeModal,
      props: {
        addRecipeForm: data.addRecipeForm
      }
    };
    modalStore.trigger({
      type: 'component', component
    });
  };

	const editRecipeModalOpen = (event: MouseEvent, row: Recipy) => {
		event.preventDefault(); event.stopPropagation();
    const component: ModalComponent = {
      ref: EditRecipeModal,
      props: {
				row
      }
    };
    modalStore.trigger({
      type: 'component', component
    });
	};
</script>

<div class="md:table-container">
	<table class="table">
		<thead>
			<tr>
				<th>Name</th>
        <th class="table-cell-fit">
					<button class="btn-icon variant-soft" on:click={addRecipeModalOpen}>
						<IconAdd />
					</button>
        </th>
			</tr>
		</thead>
		<tbody>
			{#if !recipies}
				...Loading
			{:else}
				{#each recipies as row (row.name)}
					<tr on:click={(e) => editRecipeModalOpen(e, row)}>
						<td> {row.name} </td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
