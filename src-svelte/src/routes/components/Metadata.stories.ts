import MetadataComponent from "./Metadata.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import { MockAppLayoutDecorator } from "$lib/__mocks__/decorators";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: MetadataComponent,
  title: "Screens/Dashboard/Metadata",
  argTypes: {},
  decorators: [
    TauriInvokeDecorator,
    MockAppLayoutDecorator,
    SvelteStoresDecorator,
  ],
};

const Template = ({ ...args }) => ({
  Component: MetadataComponent,
  props: args,
});

export const Loaded: StoryObj = Template.bind({}) as any;
Loaded.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
  sampleCallFiles: ["/api/sample-calls/get_system_info-linux.yaml"],
};

export const Loading: StoryObj = Template.bind({}) as any;
Loading.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
  shouldWait: true,
};
