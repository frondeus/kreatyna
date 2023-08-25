<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import RootLayout from './RootLayout.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
</script>

<Meta title="Organisms/Layouts/RootLayout" component={RootLayout} />

<Template let:args>
	<StorybookLayout isLoggedIn={args.isLoggedIn} />
</Template>

<Story name="Default" args={{ isLoggedIn: true }} />

<Story name="Logged Out" args={{ isLoggedIn: false }} />
