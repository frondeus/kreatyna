<script lang="ts">
	import EditRecipeForm from '$components/organisms/EditRecipeForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<EditRecipeForm form={data.form} relatedIngredients={new Map()} />
