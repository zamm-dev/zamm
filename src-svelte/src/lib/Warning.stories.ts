import WarningView from "./WarningView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: WarningView,
  title: "Reusable/Warning",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: WarningView,
  props: args,
});

export const Short: StoryObj = Template.bind({}) as any;
Short.args = {
  text: "This is a warning.",
};
Short.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const Long: StoryObj = Template.bind({}) as any;
Long.args = {
  text:
    "Please note that this is a warning. " +
    "It is important to pay attention to this warning, " +
    "or else the integrity of the simulation may be at stake.",
};
Long.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};
