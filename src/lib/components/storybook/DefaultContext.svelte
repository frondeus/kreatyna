<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import { setContext } from '$shared/context';
	import { writable } from 'svelte/store';

	setContext({
		page: writable({
			url: new URL('http://localhost/'),
			params: {},
			route: { id: null },
			status: 200,
			error: null,
			data: {
				session: null
			},
			form: {}
		})
	});
</script>

<slot />
