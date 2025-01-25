<script lang="ts">
	import type { PageData } from './$types';
	import EditRawForm from '$components/organisms/EditRawForm.svelte';

	export let data: PageData;

	let { id } = data;
	$: ({ id } = data);
</script>

<svelte:head>
	<title>Kreatyna - Browse Raw</title>
</svelte:head>

<EditRawForm {id} form={data.form} />
