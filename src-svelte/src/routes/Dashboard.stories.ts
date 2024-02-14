import DashboardComponent from "./Dashboard.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";

export default {
  component: DashboardComponent,
  title: "Screens/Dashboard",
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
  Component: DashboardComponent,
  props: args,
});

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_keys-openai.yaml",
    "/api/sample-calls/get_system_info-linux.yaml",
  ],
};
