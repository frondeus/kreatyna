<script lang="ts">
	import { superForm } from 'sveltekit-superforms/client';
	import type { PageData } from './$types';
	import TextInput from '$lib/components/atoms/forms/TextInput.svelte';
	import PasswordInput from '$lib/components/atoms/forms/PasswordInput.svelte';
	import Submit from '$lib/components/atoms/forms/Submit.svelte';

	export let data: PageData;
	const form = superForm(data.form);
	$: ({ enhance } = form);
</script>

<svelte:head>
	<title>Kreatyna - Sign In</title>
</svelte:head>

<form method="post" use:enhance class="space-y-4 p-2">
	<h1 class="h1 text-center">Sign in</h1>
	<TextInput {form} title="Username" field="username" />
	<PasswordInput {form} title="Password" name="password" />
	<div class="grid grid-cols-1 gap-4">
		<Submit value="Sign in" />

		<!-- <a class="btn variant-filled-surface" href="/signup">Create an account</a> -->
	</div>
</form>
