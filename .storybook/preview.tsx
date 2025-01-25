import "@/App.css";
import "@/i18next";
import Layout from "@components/_layout/Layout";
import Root from "@components/_layout/Root";
import { API } from "@lib/api";
import "@mantine/core/styles.css";
import type { Preview } from "@storybook/react";

const mockAPI: API = {
  getShoppingLists: async () => [],
  createShoppingList: async () => 1,
  getItemSuggestions: async () => [],
};

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
      const { pageLayout, layout } = parameters;
      switch (pageLayout ?? layout) {
        case "fullscreen":
        case "page":
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
            </Root>
          );
      }
    },
  ],
};

export default preview;
