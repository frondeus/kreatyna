<script lang="ts">
	import { enhance } from '$app/forms';
	import { modalStore } from '@skeletonlabs/skeleton';

	export let parent: {
		onClose: () => void;
	};
	export let resource: string;
	export let id: unknown;
	export let name: string;
</script>

{#if $modalStore[0]}
	<form
		class="form-widget card p-4 shadow-xl space-y-4"
		method="post"
		action="?/delete"
		use:enhance={() => {
			return async ({ result }) => {
				if (result.type === 'success') {
					parent.onClose();
				}
			};
		}}
	>
		<h1 class="h1">Are you sure?</h1>
		<h3 class="h3">You are about to delete {resource} "{name}".</h3>
		<p class="text-warning-500">This action cannot be undone.</p>
		<input type="hidden" name="id" value={id} />
		<button class="btn variant-filled-error input" type="submit">Yes, delete</button>
	</form>
{/if}
