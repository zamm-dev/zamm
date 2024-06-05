import ApiCallsComponent from "./ApiCalls.svelte";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";
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
        Component: MockPageTransitions,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: ApiCallsComponent,
  props: args,
});

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.args = {
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
FullPage.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: [
    "/api/sample-calls/get_api_calls-full.yaml",
    "/api/sample-calls/get_api_calls-offset.yaml",
  ],
};
