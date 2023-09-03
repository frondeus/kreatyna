<script>
	import { onMount } from 'svelte';
	import DefaultContext from './DefaultContext.svelte';

	onMount(() => {
		document.getElementsByTagName('html')[0].setAttribute('class', 'dark');
	});
</script>

<DefaultContext>
	<div class="bg-surface-900 p-6">
		<slot />
	</div>
</DefaultContext>
