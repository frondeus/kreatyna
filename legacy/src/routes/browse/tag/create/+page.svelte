<script lang="ts">
	import EditTagForm from '$components/organisms/EditTagForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Kreatyna - Create Tag</title>
</svelte:head>

<EditTagForm form={data.form} />
