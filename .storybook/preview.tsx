import "@/App.css";
import MainDecorator from "@/components/storybook/MainDecorator";
import "@/i18next";
import "@mantine/core/styles.css";
import type { Preview } from "@storybook/react";

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
      return (
        <MainDecorator parameters={parameters}>
          <Story />
        </MainDecorator>
      );
    },
  ],
};

export default preview;
