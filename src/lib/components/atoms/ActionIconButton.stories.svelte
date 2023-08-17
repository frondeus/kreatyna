<script>
import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

import IconMenu from '~icons/mdi/menu';
import ActionIconButton from "./ActionIconButton.svelte";

</script>

<Meta
    title="Atoms/ActionIconButton",
    component={ActionIconButton}
/>

<Template let:args>
    <ActionIconButton {...args}>
        <IconMenu />
    </ActionIconButton>
</Template>

<Story name="Default" args={{href: "/menu"}} />
