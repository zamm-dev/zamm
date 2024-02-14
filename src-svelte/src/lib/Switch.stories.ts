import Switch from "./Switch.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";

export default {
  component: Switch,
  title: "Reusable/Switch",
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
