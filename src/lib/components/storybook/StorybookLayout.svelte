<script lang="ts">
	import { onMount } from 'svelte';
	import DefaultContext from './DefaultContext.svelte';
	import RootLayout from '$components/organisms/layouts/RootLayout.svelte';

	export let isLoggedIn: boolean;
	export let fullscreen = false;
	onMount(() => {
		document.getElementsByTagName('html')[0].setAttribute('class', 'dark');
	});
</script>

<DefaultContext>
	<div
		class="h-full bg-surface-900"
		class:overflow-auto={!fullscreen}
		class:overflow-hidden={fullscreen}
	>
		<RootLayout {isLoggedIn}>
			<slot />
		</RootLayout>
	</div>
</DefaultContext>
