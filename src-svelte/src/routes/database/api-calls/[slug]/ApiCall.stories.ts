import ApiCallComponent from "./ApiCall.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import { KHMER_CALL, LOTS_OF_CODE_CALL } from "./sample-calls";

export default {
  component: ApiCallComponent,
  title: "Screens/Database/LLM Call",
  argTypes: {},
  decorators: [TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiCallComponent,
  props: args,
});

export const Khmer: StoryObj = Template.bind({}) as any;
Khmer.args = {
  apiCall: KHMER_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Khmer.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const LotsOfCode: StoryObj = Template.bind({}) as any;
LotsOfCode.args = {
  apiCall: LOTS_OF_CODE_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
