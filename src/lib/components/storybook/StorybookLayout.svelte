<script lang="ts">
	import { onMount } from 'svelte';
	import DefaultContext from './DefaultContext.svelte';
	import RootLayout from '$components/organisms/layouts/RootLayout.svelte';

	export let isLoggedIn: boolean;
	onMount(() => {
		document.getElementsByTagName('html')[0].setAttribute('class', 'dark');
	});
</script>

<DefaultContext>
	<div style="display: contents" class="h-full overflow-hidden">
		<RootLayout {isLoggedIn}>
			<slot />
		</RootLayout>
	</div>
</DefaultContext>
