import Button from "./ButtonView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Button,
  title: "Reusable/Button",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Button,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  text: "Simulate",
};

export const Disabled: StoryObj = Template.bind({}) as any;
Disabled.args = {
  text: "Simulate",
  disabled: true,
};
