<script lang="ts">
	import type { PageData } from './$types';
	import { superForm } from 'sveltekit-superforms/client';
	// import SuperDebug from 'sveltekit-superforms/client/SuperDebug.svelte';

	export let data: PageData;

	const { form, errors, constraints, enhance, delayed } = superForm(data.updateForm, {
		multipleSubmits: 'prevent'
	});
	const { enhance: signoutEnhance } = superForm(data.signoutForm, {
		multipleSubmits: 'prevent'
	});
	let { session } = data;
	$: ({ session } = data);
</script>

<!-- <SuperDebug data={$form} /> -->

<form class="form-widget" method="POST" action="?/update" use:enhance>
	<label class="label">
		<span>E-mail</span>
		<input
			class="input"
			title="E-mail"
			name="email"
			type="email"
			placeholder="john@example.com"
			value={session.user.email}
			disabled
		/>
	</label>

	<label class="label">
		<span>Full Name</span>
		<input
			class="input"
			title="Full Name"
			name="fullName"
			type="text"
			aria-invalid={$errors.fullName ? 'true' : undefined}
			bind:value={$form.fullName}
			{...$constraints.fullName}
		/>
	</label>

	{#if $errors.fullName}
		<span class="invalid">
			{$errors.fullName}
		</span>
	{/if}

	<input type="submit" class="input btn variant-filled" value={$delayed ? 'Updating' : 'Update'} />
</form>

<form method="post" action="?/signout" use:signoutEnhance>
	<button class="input btn variant-filled">Sign Out</button>
</form>
