import React from 'react';
import type { Preview } from "@storybook/react";
import { Root } from "../src/components/_layout/Root";
import { Layout } from '../src/components/_layout/Layout';
import '../src/i18next';
import '@mantine/core/styles.css';
import "../src/App.css";
import { APIContext, API } from '../src/api';

const mockAPI: API = {
  getShoppingLists: async () => [],
  createShoppingList: async () => 1,
  getItemSuggestions: async () => []
}

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  decorators: [
    (Story, { parameters }) => {
      // 👇 Make it configurable by reading from parameters
      console.log(parameters);
      const { pageLayout , layout } = parameters;
      switch (pageLayout ?? layout) {
        case 'fullscreen':
        case 'page':
          return (
            <Root api={mockAPI}>
                <Layout>
                  <Story />
                </Layout>
            </Root>
          );
        default:
          return (
          <Root api={mockAPI}>
              <Story />
            </Root>);
      }
    }
  ]
};

export default preview;
