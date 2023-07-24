<script lang="ts">
	import type { PageData } from './$types';
	import EditRawForm from '$lib/components/molecules/EditRawForm.svelte';

	export let data: PageData;

	let { id } = data;
	$: ({ id } = data);
</script>

<EditRawForm {id} form={data.form} />
