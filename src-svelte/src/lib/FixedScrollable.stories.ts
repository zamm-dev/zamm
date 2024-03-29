import FixedScrollableComponent from "./FixedScrollableView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: FixedScrollableComponent,
  title: "Reusable/Scrollable/Fixed",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: FixedScrollableComponent,
  props: args,
});

export const Top: StoryObj = Template.bind({}) as any;
Top.args = {
  initialPosition: "top",
};

export const Bottom: StoryObj = Template.bind({}) as any;
Bottom.args = {
  initialPosition: "bottom",
};

export const AutoScroll: StoryObj = Template.bind({}) as any;
AutoScroll.args = {
  autoscroll: true,
};
