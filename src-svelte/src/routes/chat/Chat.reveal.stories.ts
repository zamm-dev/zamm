import ChatComponent from "./Chat.svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import { conversation } from "./Chat.mock-data";

export default {
  component: ChatComponent,
  title: "Screens/Chat/Conversation",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
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
  Component: ChatComponent,
  props: args,
});

export const FullPage: StoryObj = Template.bind({}) as any;

export const FullPageConversation: StoryObj = Template.bind({}) as any;
FullPageConversation.parameters = {
  stores: {
    conversation,
  },
};
