import type { Meta, StoryObj } from '@storybook/react';
import { SuggestionItem } from './SuggestionItem';

const meta = {
    title: 'Shop/SuggestionItem',
    component: SuggestionItem,
    parameters: {
        layout: 'centered',
    },
    args: {
        name: "Raisin, seedless",
        amount: "50g",
        onAdd: () => {},
        onRemove: () => {},
    }
} satisfies Meta<typeof SuggestionItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithScale: Story = {
    args: {
        hasScale: true
    }
};

export const NoAmount: Story = {
    args: {
        amount: undefined
    }
}; 