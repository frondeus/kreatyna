<script lang="ts">
	import EditRecipeForm from '$lib/components/molecules/EditRecipeForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<EditRecipeForm form={data.form} />
