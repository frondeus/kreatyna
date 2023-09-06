<script lang="ts">
	import type { PageData } from './$types';
	import EditRecipeForm from '$components/organisms/EditRecipeForm.svelte';
	import type { SuperValidated } from 'sveltekit-superforms';
	import type { EditRecipeSchema } from '$lib/shared/domains/recipe/schema';

	export let data: PageData;

	let { form }: { form: SuperValidated<EditRecipeSchema> } = data;
	$: ({ form } = data);
</script>

<svelte:head>Kreatyna - Browse Recipes</svelte:head>
<EditRecipeForm {form} relatedIngredients={data.relatedIngredients} />
