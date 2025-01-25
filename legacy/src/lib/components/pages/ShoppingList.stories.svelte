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
				name: 'Apples',
				amount: 10,
				state: 'TODO'
			},
			{
				id: 2,
				name: 'Carrots',
				amount: 20,
				unit: 'g',
				state: 'TODO'
			},
			{
				id: 3,
				name: 'Chicken',
				amount: 300,
				unit: 'g',
				state: 'IN_CART'
			},
			{
				id: 4,
				name: 'Butter',
				amount: 1,
				state: 'BOUGHT'
			},
			{
				id: 5,
				name: 'Bell pepper',
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
