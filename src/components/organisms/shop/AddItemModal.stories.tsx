import type { Meta, StoryObj } from '@storybook/react';
import { AddItemModal } from './AddItemModal';
import { APIContext } from '../../../api';

const meta = {
    title: 'Organisms/Shop/AddItemModal',
    component: AddItemModal,
    parameters: {
        layout: 'fullscreen',
    },
    decorators: [
        (Story) => (
            <APIContext.Provider value={{
                getShoppingLists: async () => [],
                createShoppingList: async () => 1,
                getItemSuggestions: async (query) => {
                    // Simulate API delay
                    await new Promise(resolve => setTimeout(resolve, 500));
                    
                    const suggestions = [
                        { name: "Raisin, seedless", amount: "50 g" },
                        { name: "Chestnut, raw", amount: "50 g" },
                        { name: "Czekolada gorzka", hasScale: true },
                        { name: "Rye bread, dark, wholemeal, imported", amount: "373.2 g" },
                        { name: "Honey", amount: "20 g" },
                    ];
                    
                    return suggestions.filter(s => 
                        s.name.toLowerCase().includes(query.toLowerCase())
                    );
                }
            }}>
                <Story />
            </APIContext.Provider>
        )
    ]
} satisfies Meta<typeof AddItemModal>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
    args: {
        opened: true,
        onClose: () => {},
    }
};

export const Closed: Story = {
    args: {
        opened: false,
        onClose: () => {},
    }
};