<script lang="ts">
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import DayPlanItem from './DayPlanItem.svelte';
	import DefaultContext from '$components/storybook/DefaultContext.svelte';
</script>

<Meta title="Atoms/DayPlanItem" component={DayPlanItem} />

<Template let:args>
    <DefaultContext>
		<DayPlanItem {...args}>
            Content
        </DayPlanItem>
    </DefaultContext>
</Template>

<Story name="Default" source args={{meal: {
    day: { index: 1},
    slot: { index: 1},
    variant: "primary"
}}} />

