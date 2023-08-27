<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import Plan from './Plan.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
	import type { WeekPlanMeal } from '$shared/domains/plan/models';

	const dayIndices = [...Array(7).keys()].map((day) => day + 1);
	const atWork = [4, 6];
	const weekend = [1, 7];
	const onlyMeals: WeekPlanMeal[] = [
		...dayIndices.map((day) => ({
			day,
			slot: 2,
			size: 2,
			tags: [],
			name: 'Breakfast',
			recipe: null
			// recipe: 'Spinach and feta cheese omelet with a side of toasted bread'
		})),
		...dayIndices.map((day) => ({
			day,
			slot: 6,
			size: 1,
			tags: [],
			name: 'Snack',
			recipe: null
			// recipe: 'Greek yogurth with a drizzle of honey'
		})),
		// ...dayIndices
		// 	.filter((d) => d % 2 === 1)
		// 	.map((day) => ({
		// 		day,
		// 		slot: { index: 9, size: 1 },
		// 		name: 'Lunch',
		// 		// recipe: 'Chicken breast with a side of rice and vegetables',
		// 		variant: 'secondary' // Cook
		// 	})),
		...dayIndices.map((day) => ({
			day,
			slot: 10,
			size: 2,
			tags: [],
			name: 'Lunch',
			recipe: null
			// recipe: 'Chicken breast with a side of rice and vegetables',
		})),
		...dayIndices.map((day) => ({
			day,
			slot: 14,
			size: 1,
			tags: [],
			name: 'Desert',
			recipe: null
			// recipe: 'Frozen grapes'
		})),
		// ...dayIndices
		// 	.filter((d) => d % 2 === 1)
		// 	.map((day) => ({
		// 		day,
		// 		slot: { index: 16, size: 1 },
		// 		name: 'Dinner',
		// 		recipe: 'Grilled Tofu with stir-fried vegetables',
		// 		variant: 'secondary' // Cook
		// 	})),
		...dayIndices.map((day) => ({
			day,
			slot: 17,
			size: 2,
			tags: [],
			name: 'Dinner',
			recipe: null
			// recipe: 'Grilled Tofu with stir-fried vegetables',
		}))
	];
	const withWorkout: WeekPlanMeal[] = [
		...onlyMeals,
		...dayIndices
			.filter((d) => !atWork.includes(d) && !weekend.includes(d))
			.map((day) => ({
				day,
				slot: 8,
				size: 1,
				tags: [],
				name: 'Workout',
				recipe: null,
				variant: 'tertiary' // Workout
			})),
		...atWork.map((day) => ({
			day,
			tags: [],
			slot: 16,
			size: 1,
			name: 'Workout',
			recipe: null,
			variant: 'tertiary' // Workout
		}))
	];
</script>

<Meta title="Pages/Plan" component={Plan} />

<Template let:args>
	<StorybookLayout isLoggedIn={true}>
		<WithRoute href="/plan">
			<Plan {...args} />
		</WithRoute>
	</StorybookLayout>
</Template>

<Story name="Default" source args={{ weekPlan: { meals: onlyMeals } }} />

<Story name="WithWorkout" source args={{ weekPlan: { meals: withWorkout } }} />

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
