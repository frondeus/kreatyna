<script lang="ts">
	import type { SuperValidated } from 'sveltekit-superforms';
	import { superForm } from 'sveltekit-superforms/client';
	import { toastStore } from '@skeletonlabs/skeleton';
	import TextInput from '$components/atoms/forms/TextInput.svelte';
	import Submit from '$components/atoms/forms/Submit.svelte';
	import type { TagCreateOneSchema } from '$shared/domains/tags/schema';

	export let form: SuperValidated<typeof TagCreateOneSchema>;

	const parentForm = superForm(form, {
		multipleSubmits: 'prevent',
		onError: ({ result }) => {
			toastStore.trigger({
				message: result.error.message,
				background: 'variant-filled-error'
			});
		},
		onUpdated: ({ form }) => {
			if (form.valid) {
				toastStore.trigger({
					message: 'Tag saved',
					background: 'variant-filled-success'
				});
			}
		}
	});
	const { enhance, delayed } = parentForm;
	$: submitDelayed = 'Adding...';
	$: submit = 'Add';
</script>

<form class="card p-4 shadow-xl space-y-4" method="post" use:enhance>
	<h2 class="h2">Add Tag</h2>

	<TextInput form={parentForm} title="Name" field="name" />

	<Submit value={$delayed ? submitDelayed : submit} />
</form>
