<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingListItem from './ShoppingListItem.svelte';
	import StorybookAtom from '$components/storybook/StorybookAtom.svelte';
	import { userEvent, within } from '@storybook/testing-library';
</script>

<Meta
	title="Molecules/ShoppingListItem"
	component={ShoppingListItem}
	argTypes={{
		state: {
			control: {
				type: 'radio'
			},
			options: ['TODO', 'IN_CART', 'BOUGHT', 'NOT_BUING']
		}
	}}
/>

<Template let:args>
	<StorybookAtom>
		<ul>
			<ShoppingListItem {...args}>Content</ShoppingListItem>
		</ul>
	</StorybookAtom>
</Template>

<Story
	name="Default"
	source
	args={{
		id: 1,
		name: 'Test',
		amount: 1,
		unit: 'g',
		state: 'TODO'
	}}
/>

<Story
	name="When unit is not specified"
	source
	args={{
		id: 2,
		name: 'Test',
		amount: 2,
		state: 'TODO'
	}}
/>

<Story
	name="When amount is 1 and unit is not specified"
	source
	args={{
		id: 3,
		name: 'Test',
		amount: 1,
		state: 'TODO'
	}}
/>

<Story
	name="Checked"
	args={{
		id: 4,
		name: 'Chicken',
		amount: 100,
		unit: 'g',
		state: 'TODO'
	}}
	play={async ({ step, canvasElement }) => {
		const canvas = within(canvasElement);

		await step('Click on checkbox', async () => {
			await userEvent.hover(canvas.getByRole('checkbox'));
			await userEvent.click(canvas.getByRole('checkbox'));
		});
	}}
/>

<Story
	name="Unchecked"
	args={{
		id: 5,
		name: 'Chicken',
		amount: 100,
		unit: 'g',
		state: 'IN_CART'
	}}
	play={async ({ step, canvasElement }) => {
		const canvas = within(canvasElement);

		await step('Click on checkbox', async () => {
			await userEvent.click(canvas.getByRole('checkbox'));
		});
	}}
/>
