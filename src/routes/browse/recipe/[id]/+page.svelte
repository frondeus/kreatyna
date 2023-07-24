<script lang="ts">
	import type { PageData } from './$types';
	import EditRecipeForm from '$lib/components/molecules/EditRecipeForm.svelte';

	export let data: PageData;

	let { id } = data;
	$: ({ id } = data);
</script>

<EditRecipeForm {id} form={data.form} />
