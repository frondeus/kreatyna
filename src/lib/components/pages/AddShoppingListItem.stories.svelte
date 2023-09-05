<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import AddShoppingListItem from './AddShoppingListItem.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
	import { v4 as uuid } from 'uuid';
</script>

<Meta title="Pages/AddShoppingListItem" component={AddShoppingListItem} />

<Template let:args>
	<StorybookLayout isLoggedIn={true}>
		<WithRoute href="/shop/add">
			<AddShoppingListItem {...args} />
		</WithRoute>
	</StorybookLayout>
</Template>

<Story
	name="Default"
	source
	args={{
		proposals: [
			{ name: 'Chicken', amount: 0, id: uuid(), state: 'TODO' },
			{ name: 'Rice', amount: 1, id: uuid(), state: 'IN_CART' },
			{ name: 'Creatine', amount: 2, id: uuid(), state: 'NOT_BUING' },
			{ name: 'Creatine', amount: 2, id: uuid(), unit: 'g', state: 'TODO' },
			{ name: 'Carrots', amount: 10, unit: 'g', id: uuid(), state: 'BOUGHT' }
		]
	}}
	parameters={{
		viewport: {
			defaultViewport: 'sm'
		}
	}}
/>
