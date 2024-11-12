import InfoBox from "./InfoBoxView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockPageTransitionsDecorator,
  MockTransitionsDecorator,
} from "./__mocks__/decorators";

export default {
  component: InfoBox,
  title: "Reusable/InfoBox",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: InfoBox,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  title: "Simulation",
  maxWidth: "50rem",
};

export const MountTransition: StoryObj = Template.bind({}) as any;
MountTransition.args = {
  title: "Simulation",
  preDelay: 0,
  maxWidth: "50rem",
};
MountTransition.decorators = [SvelteStoresDecorator, MockTransitionsDecorator];

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  title: "Simulation",
  preDelay: 0,
  maxWidth: "50rem",
};
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};
SlowMotion.decorators = [SvelteStoresDecorator, MockTransitionsDecorator];

export const Motionless: StoryObj = Template.bind({}) as any;
Motionless.args = {
  title: "Simulation",
  maxWidth: "50rem",
};
Motionless.parameters = {
  preferences: {
    animationsOn: false,
  },
};
Motionless.decorators = [SvelteStoresDecorator, MockTransitionsDecorator];

export const Transparent: StoryObj = Template.bind({}) as any;
Transparent.args = {
  title: "Simulation",
  maxWidth: "50rem",
};
Transparent.parameters = {
  preferences: {
    animationsOn: false,
    backgroundAnimation: false,
    transparencyOn: true,
  },
};
Transparent.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];
