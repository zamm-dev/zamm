import PageTransitionView from "./PageTransitionView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: PageTransitionView,
  title: "Layout/Page Transitions",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: PageTransitionView,
  props: args,
});

export const Default: StoryObj = Template.bind({}) as any;
Default.parameters = {
  preferences: {
    animationsOn: true,
  },
};

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.parameters = {
  preferences: {
    animationsOn: true,
    animationSpeed: 0.1,
  },
};

export const Subpath: StoryObj = Template.bind({}) as any;
Subpath.args = {
  routeBAddress: "/a/subpath",
};
Subpath.parameters = {
  preferences: {
    animationsOn: true,
  },
};

export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.parameters = {
  preferences: {
    animationsOn: false,
  },
};
