<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import Plan from './DayPlan.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
	import type { WeekPlanMealWithId } from '$shared/domains/plan/models';

	const dayIndices = [5];
	const onlyMeals: WeekPlanMealWithId[] = [
		...dayIndices.map((day) => ({
			id: 0,
			day,
			slot: 2,
			size: 2,
			tags: [],
			name: 'Breakfast',
			recipe: {
				id: 0 as unknown as bigint,
				name: 'Spinach and feta cheese omelet with a side of toasted bread',
				tags: []
			}
		})),
		...dayIndices.map((day) => ({
			id: 1,
			day,
			slot: 6,
			size: 1,
			tags: [],
			name: 'Snack',
			recipe: {
				id: 1 as unknown as bigint,
				name: 'Greek yogurth with a drizzle of honey',
				tags: []
			}
		})),
		...dayIndices.map((day) => ({
			id: 2,
			day,
			slot: 10,
			size: 2,
			tags: [],
			name: 'Lunch',
			recipe: {
				id: 2 as unknown as bigint,
				name: 'Chicken breast with a side of rice and vegetables',
				tags: []
			}
		})),
		...dayIndices.map((day) => ({
			id: 3,
			day,
			slot: 14,
			size: 1,
			tags: [],
			name: 'Desert',
			recipe: {
				id: 3 as unknown as bigint,
				name: 'Frozen grapes',
				tags: []
			}
		})),
		...dayIndices.map((day) => ({
			id: 4,
			day,
			slot: 17,
			size: 2,
			tags: [],
			name: 'Dinner',
			recipe: {
				id: 4 as unknown as bigint,
				name: 'Grilled Tofu with stir-fried vegetables',
				tags: []
			}
		}))
	];

	const currentDate = new Date('2022-01-06T10:00:00.000Z');
	currentDate.setHours(0, 0, 0, 0);
</script>

<Meta
	title="Pages/DayPlan"
	component={Plan}
	args={{
		weekPlan: {
			meals: onlyMeals,
			week: 2,
			year: 2022
		},
		currentDate
	}}
/>

<Template let:args>
	<StorybookLayout isLoggedIn={true}>
		<WithRoute href="/plan">
			<Plan {...args} />
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
