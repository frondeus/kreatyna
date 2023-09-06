<script lang="ts">
	import EditRecipeForm from '$components/organisms/EditRecipeForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>Kreatyna - Create Recipe</svelte:head>

<EditRecipeForm form={data.form} relatedIngredients={new Map()} />
