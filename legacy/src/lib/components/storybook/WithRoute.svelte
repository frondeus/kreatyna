<script lang="ts">
	import { getContext } from '$shared/context';

	export let href: string;

	const page = getContext().page;
	$page.url = new URL(href, 'http://localhost/');
</script>

<slot />
