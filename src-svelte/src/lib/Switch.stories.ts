import Switch from "./Switch.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "./__mocks__/decorators";

export default {
  component: Switch,
  title: "Reusable/Switch",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Switch,
  props: args,
});

export const On: StoryObj = Template.bind({}) as any;
On.args = {
  label: "Simulation",
  toggledOn: true,
};
On.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};
On.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

export const Off: StoryObj = Template.bind({}) as any;
Off.args = {
  label: "Simulation",
  toggledOn: false,
};
Off.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};
Off.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

export const Fast: StoryObj = Template.bind({}) as any;
Fast.args = {
  label: "Simulation",
  toggledOn: false,
};
Fast.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
  preferences: {
    animationSpeed: 1,
  },
};
Fast.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  label: "Simulation",
  toggledOn: false,
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
