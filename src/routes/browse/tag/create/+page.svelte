<script lang="ts">
	import EditTagForm from '$components/organisms/EditTagForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<EditTagForm form={data.form} />
