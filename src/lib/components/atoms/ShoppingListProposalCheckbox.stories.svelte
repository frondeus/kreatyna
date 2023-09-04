<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingListProposalCheckbox from './ShoppingListProposalCheckbox.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
</script>

<Meta title="Atoms/ShoppingListProposalCheckbox" component={ShoppingListProposalCheckbox} />

<Template let:args>
	<StorybookAtom>
		<ShoppingListProposalCheckbox {...args} />
	</StorybookAtom>
</Template>

<Story name="Default" source args={{}} />

<Story
	name="Added"
	source
	args={{
		state: 'added'
	}}
/>
