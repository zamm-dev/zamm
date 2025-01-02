import UnloadedApiCall from "./UnloadedApiCall.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import { MockPageTransitionsDecorator } from "$lib/__mocks__/decorators";

export default {
  component: UnloadedApiCall,
  title: "Screens/Database/LLM Call",
  argTypes: {},
  decorators: [TauriInvokeDecorator, SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: UnloadedApiCall,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  id: "c13c1e67-2de3-48de-a34c-a32079c03316",
  showActions: false,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Regular.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_call-continue-conversation.yaml",
  ],
};

export const Variant: StoryObj = Template.bind({}) as any;
Variant.args = {
  id: "7a35a4cf-f3d9-4388-bca8-2fe6e78c9648",
  showActions: false,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Variant.parameters = {
  sampleCallFiles: ["/api/sample-calls/get_api_call-edit.yaml"],
};

export const UnknownProviderPrompt: StoryObj = Template.bind({}) as any;
UnknownProviderPrompt.args = {
  id: "037b28dd-6f24-4e68-9dfb-3caa1889d886",
  showActions: false,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
UnknownProviderPrompt.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_call-unknown-provider-prompt.yaml",
  ],
};

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.args = {
  id: "c13c1e67-2de3-48de-a34c-a32079c03316",
  showActions: true,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
FullPage.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_api_call-continue-conversation.yaml",
  ],
};
FullPage.decorators = [MockPageTransitionsDecorator];
