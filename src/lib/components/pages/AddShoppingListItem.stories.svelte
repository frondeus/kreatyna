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
			{ name: 'Chicken', amount: 0, id: uuid() },
			{ name: 'Rice', amount: 1, id: uuid() },
			{ name: 'Creatine', amount: 2, id: uuid() }
		]
	}}
	parameters={{
		viewport: {
			defaultViewport: 'sm'
		}
	}}
/>
