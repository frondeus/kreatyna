<script lang="ts">
	import type { PageData } from './$types';
	import EditRawForm from '$components/organisms/EditRawForm.svelte';

	export let data: PageData;
</script>

<svelte:head>
	<title>Kreatyna - Create Raw</title>
</svelte:head>

<EditRawForm form={data.form} />
