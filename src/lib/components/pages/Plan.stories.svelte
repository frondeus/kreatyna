<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import Plan from './Plan.svelte';
	import StorybookLayout from '$components/storybook/StorybookLayout.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
	import type { MealForDay } from '$shared/domains/plan/models';

	const dayIndices = [...Array(7).keys()].map((day) => day + 1);
	const atWork = [4, 6];
	const weekend = [1, 7];
	const onlyMeals: MealForDay[] = [
		...dayIndices.map((day) => ({
			day,
			slot: { index: 2, size: 2 },
			tags: [],
			name: 'Breakfast'
			// recipe: 'Spinach and feta cheese omelet with a side of toasted bread'
		})),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 6 },
			tags: [],
			name: 'Snack'
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
			slot: { index: 10, size: 2 },
			tags: [],
			name: 'Lunch'
			// recipe: 'Chicken breast with a side of rice and vegetables',
		})),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 14 },
			tags: [],
			name: 'Desert'
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
			slot: { index: 17, size: 2 },
			tags: [],
			name: 'Dinner'
			// recipe: 'Grilled Tofu with stir-fried vegetables',
		}))
	];
	const withWorkout: MealForDay[] = [
		...onlyMeals,
		...dayIndices
			.filter((d) => !atWork.includes(d) && !weekend.includes(d))
			.map((day) => ({
				day,
				slot: { index: 8 },
				tags: [],
				name: 'Workout',
				variant: 'tertiary' // Workout
			})),
		...atWork.map((day) => ({
			day,
			tags: [],
			slot: { index: 16 },
			name: 'Workout',
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

<Story name="Default" source args={{ meals: onlyMeals }} />

<Story name="WithWorkout" source args={{ meals: withWorkout }} />

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
