import BackgroundComponent from "./BackgroundUI.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import BackgroundUIView from "./BackgroundUIView.svelte";

export default {
  component: BackgroundComponent,
  title: "Layout/Background",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: BackgroundUIView,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: BackgroundComponent,
  props: args,
});

export const Static: StoryObj = Template.bind({}) as any;
Static.args = {
  animated: false,
};
export const Dynamic: StoryObj = Template.bind({}) as any;
Dynamic.args = {
  animated: true,
};

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  animated: true,
};
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};
