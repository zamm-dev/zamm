import ActionsComponent from "./Actions.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";

export default {
  component: ActionsComponent,
  title: "Screens/Database/LLM Call/Actions",
  argTypes: {},
  decorators: [TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ActionsComponent,
  props: args,
});

export const Wide: StoryObj = Template.bind({}) as any;
Wide.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const Narrow: StoryObj = Template.bind({}) as any;
Narrow.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};
