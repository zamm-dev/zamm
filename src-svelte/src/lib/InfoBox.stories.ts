import InfoBox from "./InfoBoxView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
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
Regular.decorators = [MockAppLayoutDecorator];

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.args = {
  title: "Simulation",
  preDelay: 0,
  maxWidth: "50rem",
};
FullPage.parameters = {
  preferences: {
    animationSpeed: 1,
  },
};
FullPage.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];

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
SlowMotion.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];
