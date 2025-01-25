import type { Preview } from "@storybook/svelte";
import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
import "@skeletonlabs/skeleton/styles/skeleton.css";
import { INITIAL_VIEWPORTS } from "@storybook/addon-viewport";

export const customViewports = {
  fullHD: {
    name: "Full HD",
    styles: {
      width: "1920px",
      height: "1080px",
    },
  },
  p1440: {
    name: "1440p",
    styles: {
      width: "2560px",
      height: "1440px",
    },
  },
  sm: {
    name: "Small",
    styles: {
      width: "640px",
      height: "1136px",
    },
  },
  md: {
    name: "Medium",
    styles: {
      width: "768px",
      height: "1024px",
    },
  },
  lg: {
    name: "Large",
    styles: {
      width: "1024px",
      height: "768px",
    },
  },
  xl: {
    name: "Extra Large",
    styles: {
      width: "1280px",
      height: "960px",
    },
  },
  xxl: {
    name: "Extra Extra Large",
    styles: {
      width: "1536px",
      height: "1920px",
    },
  },
};
const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    // layout: 'fullscreen',
    viewport: {
      viewports: {
        ...INITIAL_VIEWPORTS,
        ...customViewports,
      },
    },
  },
};

export default preview;
