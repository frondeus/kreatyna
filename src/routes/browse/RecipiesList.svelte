<script lang="ts">
	import { modalStore, type ModalComponent } from "@skeletonlabs/skeleton";
	import type { PageData } from "./$types";
  import AddRecipeModal from "./AddRecipeModal.svelte";
	import IconAdd from '~icons/mdi/add';


  export let data: PageData;
  let { supabase, session, recipies } = data;
  $: ({ supabase, session, recipies } = data);

// const recipies = realtime(
//   supabase,
// data.recipies ?? [],
// 'recipies',
// (row: Recip)
// )
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
					<tr>
						<td> {row.name} </td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
