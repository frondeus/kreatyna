import type { Meta, StoryObj } from "@storybook/svelte";
import Plan from "./plan.svelte";

const meta = {
    title: 'Pages/Plan',
    component: Plan,
} satisfies Meta<Plan>;
export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        isLoggedIn: true
    },
};

export const LoggedOut: Story = {
    args: {
        isLoggedIn: false
    },
};