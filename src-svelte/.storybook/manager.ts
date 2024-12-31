import { addons } from "@storybook/manager-api";

// https://github.com/storybookjs/storybook/discussions/26058
addons.register("custom-panel", (api) => {
  api.togglePanel(false);
});
