<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingListProposalUnit from './ShoppingListProposalUnit.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
</script>

<Meta title="Atoms/ShoppingListProposalUnit" component={ShoppingListProposalUnit} />

<Template let:args>
	<StorybookAtom>
		<ShoppingListProposalUnit {...args} />
	</StorybookAtom>
</Template>

<Story
	name="ReadOnly"
	source
	args={{
		amount: 100,
		unit: 'g',
		editable: false
	}}
/>

<Story
	name="Editable"
	source
	args={{
		amount: 100,
		unit: 'g',
		editable: true
	}}
/>

<Story
	name="Without unit"
	source
	args={{
		amount: 5,
		editable: true
	}}
/>

<Story
	name="Without unit & with amount 1"
	source
	args={{
		amount: 1,
		editable: true
	}}
/>

<Story
	name="With unit & with amount 1"
	source
	args={{
		amount: 1,
		unit: 'g',
		editable: true
	}}
/>
