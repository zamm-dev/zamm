import ScrollableComponent from "./ScrollableView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ScrollableComponent,
  title: "Reusable/Scrollable/Growable",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ScrollableComponent,
  props: args,
});

export const Small: StoryObj = Template.bind({}) as any;
Small.args = {
  initialPosition: "top",
};
Small.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const Large: StoryObj = Template.bind({}) as any;
Large.args = {
  initialPosition: "top",
};
Large.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};
