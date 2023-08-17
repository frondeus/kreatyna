<script>
	import { setContext } from 'svelte';
	import { page } from '$app/stores';
	import RootLayout from '$components/organisms/layouts/RootLayout.svelte';

	setContext('url', $page.url);

	export let data;
	let session = data.session;
	$: ({ session } = data);
</script>

<RootLayout isLoggedIn={!!session}>
	<slot />
</RootLayout>
