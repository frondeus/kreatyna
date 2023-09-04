<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import FloatingButton from './FloatingButton.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
	import IconPlus from '~icons/mdi/plus';
</script>

<Meta title="Atoms/Buttons/FloatingButton" component={FloatingButton} />

<Template let:args>
	<StorybookAtom>
		<FloatingButton {...args}>
			<IconPlus />
		</FloatingButton>
	</StorybookAtom>
</Template>

<Story
	name="Default"
	source
	args={{}}
	parameters={{
		viewport: {
			defaultViewport: 'sm'
		}
	}}
/>
