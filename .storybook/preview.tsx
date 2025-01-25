import React from 'react';
import type { Preview } from "@storybook/react";
import { MantineProvider } from "@mantine/core";
import { ErrorBoundary } from 'react-error-boundary';
import { ErrorPage } from "../src/layout/Error";
import { Layout } from '../src/layout/Layout';
import '../src/i18next';
import '@mantine/core/styles.css';
import "../src/App.css";

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
            <ErrorBoundary FallbackComponent={ErrorPage}>
              <MantineProvider defaultColorScheme="dark">
                <Layout>
                  <Story />
                </Layout>
              </MantineProvider>
            </ErrorBoundary>
          );
        default:
          // In the default case, don't apply a layout
          return (<ErrorBoundary FallbackComponent={ErrorPage}>
          <MantineProvider defaultColorScheme="dark">
            <Story />
          </MantineProvider>
        </ErrorBoundary>);
      }
    }
  ]
};

export default preview;
