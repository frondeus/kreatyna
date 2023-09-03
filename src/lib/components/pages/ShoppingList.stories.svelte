<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import ShoppingList from './ShoppingList.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
</script>

<Meta
	title="Pages/ShoppingList"
	component={ShoppingList}
	args={{
		items: [
			{
				id: 1,
				name: 'Test',
				amount: 1,
				unit: 'g',
				state: 'TODO'
			},
			{
				id: 2,
				name: 'Test 2',
				amount: 2,
				unit: 'g',
				state: 'TODO'
			},
			{
				id: 3,
				name: 'Test 3',
				amount: 3,
				unit: 'g',
				state: 'IN_CART'
			},
			{
				id: 4,
				name: 'Test 4',
				amount: 1,
				state: 'BOUGHT'
			},
			{
				id: 5,
				name: 'Test 5',
				amount: 5,
				state: 'NOT_BUING'
			}
		]
	}}
/>

<Template let:args>
	<StorybookLayout isLoggedIn={true}>
		<WithRoute href="/shop">
			<ShoppingList {...args} />
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
