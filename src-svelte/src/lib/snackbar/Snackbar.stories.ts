import SnackbarView from "./SnackbarView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: SnackbarView,
  title: "Layout/Snackbar",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: SnackbarView,
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

export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.parameters = {
  preferences: {
    animationsOn: false,
  },
};
