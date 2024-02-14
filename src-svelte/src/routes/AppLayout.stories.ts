import AppLayout from "./AppLayout.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: AppLayout,
  title: "Layout/App",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: AppLayout,
  props: args,
});

export const Dynamic: StoryObj = Template.bind({}) as any;
Dynamic.args = {
  currentRoute: "/",
};
Dynamic.parameters = {
  preferences: {
    backgroundAnimation: true,
  },
};

export const Static: StoryObj = Template.bind({}) as any;
Static.args = {
  currentRoute: "/",
};
Static.parameters = {
  preferences: {
    backgroundAnimation: false,
  },
};
