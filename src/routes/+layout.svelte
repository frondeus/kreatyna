<script>
	import { page } from '$app/stores';
	import RootLayout from '$components/organisms/layouts/RootLayout.svelte';
	import { setContext } from '$shared/context';
	import { onMount } from 'svelte';
	import { pwaInfo } from 'virtual:pwa-info';

	$: webManifestLink = pwaInfo ? pwaInfo.webManifest.linkTag : '';

	setContext({ page });

	export let data;
	let session = data.session;
	$: ({ session } = data);

	onMount(async () => {
		if (pwaInfo) {
			const { registerSW } = await import('virtual:pwa-register');
			registerSW({
				immediate: true,
				onRegistered(r) {
					console.log('Registered', r);
				},
				onRegisterError(e) {
					console.error(e);
				}
			});
		}
	});
</script>

<svelte:head>
	<title>Kreatyna</title>
	<meta name="theme-color" content="rgb(59 130 246 / .5)" />
	{@html webManifestLink}
</svelte:head>

<RootLayout isLoggedIn={!!session}>
	<slot />
</RootLayout>
