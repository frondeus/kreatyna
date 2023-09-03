<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingListCheckbox from './ShoppingListCheckbox.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
</script>

<Meta title="Atoms/ShoppingListCheckbox" component={ShoppingListCheckbox} />

<Template let:args>
	<StorybookAtom>
		<ShoppingListCheckbox {...args} />
	</StorybookAtom>
</Template>

<Story name="Default" source args={{}} />

<Story
	name="Checked"
	source
	args={{
		state: 'checked'
	}}
/>

<Story
	name="Crossed"
	source
	args={{
		state: 'crossed'
	}}
/>

<Story
	name="Archived"
	source
	args={{
		state: 'archived'
	}}
/>

<Story
	name="Processing"
	source
	args={{
		state: 'processing'
	}}
/>
