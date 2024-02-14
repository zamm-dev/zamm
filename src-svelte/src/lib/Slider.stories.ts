import Slider from "./Slider.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";

export default {
  component: Slider,
  title: "Reusable/Slider",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: MockAppLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: Slider,
  props: args,
});

export const TinyPhoneScreen: StoryObj = Template.bind({}) as any;
TinyPhoneScreen.args = {
  label: "Simulation",
  max: 10,
  value: 5,
};
TinyPhoneScreen.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const TinyPhoneScreenWithLongLabel: StoryObj = Template.bind({}) as any;
TinyPhoneScreenWithLongLabel.args = {
  label: "Extra Large Simulation",
  max: 10,
  value: 5,
};
TinyPhoneScreenWithLongLabel.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.args = {
  label: "Simulation",
  max: 10,
  value: 5,
};
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  label: "Extra Large Simulation",
  max: 10,
  value: 5,
};
SlowMotion.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
  preferences: {
    animationSpeed: 0.1,
  },
};
