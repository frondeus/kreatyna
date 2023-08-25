<script>
	import { page } from '$app/stores';
	import RootLayout from '$components/organisms/layouts/RootLayout.svelte';
	import { setContext } from '$shared/context';

	setContext({ page });

	export let data;
	let session = data.session;
	$: ({ session } = data);
</script>

<RootLayout isLoggedIn={!!session}>
	<slot />
</RootLayout>
