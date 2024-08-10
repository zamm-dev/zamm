import ApiCallComponent from "./ApiCallDisplay.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import {
  CONTINUE_CONVERSATION_CALL,
  KHMER_CALL,
  LOTS_OF_CODE_CALL,
  VARIANT_CALL,
  UNKNOWN_PROVIDER_PROMPT_CALL,
} from "./sample-calls";

export default {
  component: ApiCallComponent,
  title: "Screens/LLM Call/Individual",
  argTypes: {},
  decorators: [TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiCallComponent,
  props: args,
});

export const Narrow: StoryObj = Template.bind({}) as any;
Narrow.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};

export const Wide: StoryObj = Template.bind({}) as any;
Wide.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};

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

export const Variant: StoryObj = Template.bind({}) as any;
Variant.args = {
  apiCall: VARIANT_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};

export const UnknownProviderPrompt: StoryObj = Template.bind({}) as any;
UnknownProviderPrompt.args = {
  apiCall: UNKNOWN_PROVIDER_PROMPT_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
