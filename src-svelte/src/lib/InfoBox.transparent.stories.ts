import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import TransparentInfoBoxView from "./TransparentInfoBoxView.svelte";

export default {
  component: TransparentInfoBoxView,
  title: "Reusable/InfoBox",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: TransparentInfoBoxView,
  props: args,
});

export const Transparent: StoryObj = Template.bind({}) as any;
Transparent.args = {
  title: "Simulation",
  maxWidth: "50rem",
};
Transparent.parameters = {
  showBackground: true,
  preferences: {
    animationsOn: false,
    backgroundAnimation: false,
    transparencyOn: true,
  },
};
