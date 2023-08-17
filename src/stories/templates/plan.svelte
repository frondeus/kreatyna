<script lang="ts">
	import RootLayout from "$components/organisms/layouts/RootLayout.svelte";
	import StorybookLayout from "$components/organisms/layouts/StorybookLayout.svelte";

	import { setContext } from "svelte";

    export let isLoggedIn: boolean = true;
    setContext('url', new URL("http://localhost/plan"));
</script>

<StorybookLayout>
    <RootLayout isLoggedIn={isLoggedIn}>
        Test
    </RootLayout>
</StorybookLayout>
