import ApiCallImport from "./ApiCallImport.svelte";
import type { StoryObj } from "@storybook/svelte";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "$lib/__mocks__/decorators";

export default {
  component: ApiCallImport,
  title: "Screens/Database/LLM Call/Import",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ApiCallImport,
  props: args,
});

export const Static: StoryObj = Template.bind({}) as any;
Static.decorators = [MockAppLayoutDecorator];
Static.parameters = {
  fullHeight: true,
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const MountTransition: StoryObj = Template.bind({}) as any;
MountTransition.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};
MountTransition.decorators = [MockPageTransitionsDecorator];
