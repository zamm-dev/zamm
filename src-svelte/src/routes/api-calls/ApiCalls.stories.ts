import ApiCallsComponent from "./ApiCalls.svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";

export default {
  component: ApiCallsComponent,
  title: "Screens/LLM Call/List",
  argTypes: {},
  decorators: [
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockFullPageLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: ApiCallsComponent,
  props: args,
});

export const Empty: StoryObj = Template.bind({}) as any;
Empty.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: ["/api/sample-calls/get_api_calls-empty.yaml"],
};

export const Small: StoryObj = Template.bind({}) as any;
Small.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: ["/api/sample-calls/get_api_calls-small.yaml"],
};

export const Full: StoryObj = Template.bind({}) as any;
Full.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: [
    "/api/sample-calls/get_api_calls-full.yaml",
    "/api/sample-calls/get_api_calls-offset.yaml",
  ],
};
