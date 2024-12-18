import Slider from "./Slider.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "./__mocks__/decorators";

export default {
  component: Slider,
  title: "Reusable/Slider",
  argTypes: {},
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
TinyPhoneScreen.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

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
TinyPhoneScreenWithLongLabel.decorators = [
  SvelteStoresDecorator,
  MockAppLayoutDecorator,
];

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
Tablet.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

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
SlowMotion.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];
