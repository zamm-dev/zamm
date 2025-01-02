import DatabaseView from "./DatabaseView.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import { MockAppLayoutDecorator } from "$lib/__mocks__/decorators";

export default {
  component: DatabaseView,
  title: "Screens/Database/List",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    TauriInvokeDecorator,
    MockAppLayoutDecorator,
  ],
  parameters: {
    fullHeight: true,
  },
};

const Template = ({ ...args }) => ({
  Component: DatabaseView,
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
Small.args = {
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Small.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: [
    "/api/sample-calls/get_api_calls-small.yaml",
    "/api/sample-calls/get_terminal_sessions-small.yaml",
  ],
};

export const Full: StoryObj = Template.bind({}) as any;
Full.args = {
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Full.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
  sampleCallFiles: [
    "/api/sample-calls/get_api_calls-full.yaml",
    "/api/sample-calls/get_api_calls-offset.yaml",
    "/api/sample-calls/get_terminal_sessions-small.yaml",
  ],
};
