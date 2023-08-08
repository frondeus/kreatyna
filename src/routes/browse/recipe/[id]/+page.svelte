<script lang="ts">
	import type { PageData } from './$types';
	import EditRecipeForm from '$lib/components/molecules/EditRecipeForm.svelte';
	import type { SuperValidated } from 'sveltekit-superforms';
	import type { EditRecipeSchema } from '$lib/shared/domains/recipe/schema';

	export let data: PageData;

	let { id, form }: { id: number; form: SuperValidated<EditRecipeSchema> } = data;
	$: ({ id, form } = data);
</script>

<EditRecipeForm {id} {form} relatedIngredients={data.relatedIngredients} />
