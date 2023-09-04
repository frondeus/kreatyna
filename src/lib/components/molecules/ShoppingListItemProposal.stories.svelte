<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingListItemProposal from './ShoppingListItemProposal.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
</script>

<Meta title="Molecules/ShoppingListItemProposal" component={ShoppingListItemProposal} />

<Template let:args>
	<StorybookAtom>
		<ShoppingListItemProposal {...args} />
	</StorybookAtom>
</Template>

<Story
	name="Default"
	source
	args={{
		name: 'Chicken'
	}}
/>

<Story
	name="Checked"
	source
	args={{
		name: 'Chicken',
		amount: 1
	}}
/>

<Story
	name="More than one"
	source
	args={{
		name: 'Chicken',
		amount: 2
	}}
/>
