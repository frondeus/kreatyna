
import type { Meta, StoryObj } from '@storybook/react';
// import { expect, userEvent, within } from '@storybook/test';

import { APIContext } from '../../api';
import { ShopPage } from './Shop';

const meta = {
    title: 'Pages/Shop',
    component: ShopPage,
    parameters: {
        // More on how to position stories at: https://storybook.js.org/docs/configure/story-layout
        layout: 'fullscreen',
    },
} satisfies Meta<typeof ShopPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NoLists: Story = {};
export const WithLists: Story = {
    // args: {
    //     api: {
    //         getShoppingLists: async () => [{
    //             id: 1,
    //             title: 'Test List',
    //             kind: 'shoppingList',
    //             items: []
    //         }],
    //         createShoppingList: async () => 2
    //     }
    // }
    render: () => <APIContext.Provider value={{
            getShoppingLists: async () => [{
                id: 1,
                title: 'Test List',
                kind: 'shoppingList',
                items: []
            }],
            createShoppingList: async () => 2,
            getItemSuggestions: async () => []
        }}>
            <ShopPage />
        </APIContext.Provider>
};

// More on interaction testing: https://storybook.js.org/docs/writing-tests/interaction-testing
// export const LoggedIn: Story = {
//   play: async ({ canvasElement }) => {
//     const canvas = within(canvasElement);
//     const loginButton = canvas.getByRole('button', { name: /Log in/i });
//     await expect(loginButton).toBeInTheDocument();
//     await userEvent.click(loginButton);
//     await expect(loginButton).not.toBeInTheDocument();

//     const logoutButton = canvas.getByRole('button', { name: /Log out/i });
//     await expect(logoutButton).toBeInTheDocument();
//   },
// };