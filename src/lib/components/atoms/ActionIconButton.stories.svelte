<script>
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import IconMenu from '~icons/mdi/menu';
	import ActionIconButton from './ActionIconButton.svelte';
	import WithRoute from '$components/storybook/WithRoute.svelte';
	import DefaultContext from '$components/storybook/DefaultContext.svelte';
</script>

<Meta title="Atoms/ActionIconButton" component={ActionIconButton} />

<Template let:args>
	<DefaultContext>
		<ActionIconButton {...args}>
			<IconMenu />
		</ActionIconButton>
	</DefaultContext>
</Template>

<Story name="Default" source args={{ href: '/menu' }} />

<Story name="Active" args={{ href: '/menu' }}>
	<DefaultContext>
		<WithRoute href="/menu">
			<ActionIconButton href="/menu">
				<IconMenu />
			</ActionIconButton>
		</WithRoute>
	</DefaultContext>
</Story>
