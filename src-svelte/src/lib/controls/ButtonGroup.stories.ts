import ButtonGroupComponent from "./ButtonGroupView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ButtonGroupComponent,
  title: "Reusable/Button/Group",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ButtonGroupComponent,
  props: args,
});

export const Wide: StoryObj = Template.bind({}) as any;
Wide.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const Narrow: StoryObj = Template.bind({}) as any;
Narrow.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};
