import ActionsComponent from "./Actions.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: ActionsComponent,
  title: "Screens/Database/LLM Call/Actions",
  argTypes: {},
  decorators: [TauriInvokeDecorator, SvelteStoresDecorator],
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
