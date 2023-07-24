<script lang="ts">
	import type { PageData } from './$types';
	import EditRawForm from '$lib/components/molecules/EditRawForm.svelte';

	export let data: PageData;
</script>

<EditRawForm form={data.form} />
