<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import Plan from './plan.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
</script>

<Meta title="Pages/Plan" component={Plan} />

<Template let:args>
	<StorybookLayout isLoggedIn={true}>
		<WithRoute href="/plan">
			<Plan {...args} />
		</WithRoute>
	</StorybookLayout>
</Template>

<Story name="Default" source />

<Story
	name="Small"
	source
	parameters={{
		viewport: {
			defaultViewport: 'sm'
		}
	}}
/>

<Story
	name="Medium"
	source
	parameters={{
		viewport: {
			defaultViewport: 'md'
		}
	}}
/>

<Story
	name="Large"
	source
	parameters={{
		viewport: {
			defaultViewport: 'lg'
		}
	}}
/>

<Story
	name="Extra Large"
	source
	parameters={{
		viewport: {
			defaultViewport: 'xl'
		}
	}}
/>

<Story
	name="XXL"
	source
	parameters={{
		viewport: {
			defaultViewport: 'xxl'
		}
	}}
/>
