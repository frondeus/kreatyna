<script lang="ts">
	import type { PageData } from './$types';
	import EditRawForm from '$components/organisms/EditRawForm.svelte';

	export let data: PageData;
</script>

<EditRawForm form={data.form} />
