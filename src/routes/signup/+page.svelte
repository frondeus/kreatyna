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

<h1 class="h1">Sign up</h1>
<form method="post" use:enhance class="space-y-4">
	<TextInput {form} title="Username" name="username" />
	<PasswordInput {form} title="Password" name="password" />
	<PasswordInput {form} title="Confirm Password" name="passwordConfirm" />
	<div class="grid grid-cols-2 gap-4">
		<Submit value="Sign up" />

		<a class="btn variant-filled-primary" href="/login"> Sign in</a>
	</div>
</form>
