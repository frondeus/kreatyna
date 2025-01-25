
import type { Meta, StoryObj } from '@storybook/react';
// import { expect, userEvent, within } from '@storybook/test';

import { APIContext } from '../../../api';
import { AddItemModal } from './AddItemModal';

const meta = {
    title: 'Shop/AddItemModal',
    component: AddItemModal,
    parameters: {
        // More on how to position stories at: https://storybook.js.org/docs/configure/story-layout
        layout: 'fullscreen',
    },
    args: {
        opened: true,
        onClose: () => {}
    }
} satisfies Meta<typeof AddItemModal>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Empty: Story = {
    render: (props) => <APIContext.Provider value={{
            getShoppingLists: async () => [{
                id: 1,
                title: 'Test List',
                kind: 'shoppingList',
                items: []
            }],
            createShoppingList: async () => 2
        }}>
            <AddItemModal {...props} />
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