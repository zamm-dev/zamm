import "../src/routes/styles.css";
import { MINIMAL_VIEWPORTS } from "@storybook/addon-viewport";

/** @type { import('@storybook/svelte').Preview } */
const preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    viewport: {
      viewports: {
        ...MINIMAL_VIEWPORTS,
        smallTablet: {
          name: "Small Tablet",
          styles: {
            width: "834px",
            height: "650px",
          },
        },
      },
    },
  },
  decorators: [],
};

export default preview;
