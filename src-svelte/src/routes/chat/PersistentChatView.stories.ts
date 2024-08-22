import ChatComponent from "./PersistentChatView.svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import type { StoryFn, StoryObj } from "@storybook/svelte";

export default {
  component: ChatComponent,
  title: "Screens/Chat/Conversation",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
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
  Component: ChatComponent,
  props: args,
});

export const Remountable: StoryObj = Template.bind({}) as any;
