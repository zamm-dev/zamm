import ApiCallImport from "./ApiCallImport.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import { MockTransitionsDecorator } from "$lib/__mocks__/decorators";

export default {
  component: ApiCallImport,
  title: "Screens/Database/LLM Call/Import",
  argTypes: {},
  decorators: [
    (story: StoryFn) => {
      return {
        Component: MockFullPageLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: ApiCallImport,
  props: args,
});

export const Static: StoryObj = Template.bind({}) as any;
Static.parameters = {
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
MountTransition.decorators = [MockTransitionsDecorator];
