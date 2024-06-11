import ApiCallEditorComponent from "./ApiCallEditor.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import { CONTINUE_CONVERSATION_CALL } from "../[slug]/sample-calls";

export default {
  component: ApiCallEditorComponent,
  title: "Screens/LLM Call/New",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiCallEditorComponent,
  props: args,
});

export const Blank: StoryObj = Template.bind({}) as any;

export const EditContinuedConversation: StoryObj = Template.bind({}) as any;
EditContinuedConversation.parameters = {
  stores: {
    apiCallEditing: {
      canonicalRef: {
        id: "c13c1e67-2de3-48de-a34c-a32079c03316",
        snippet:
          "Sure, here's a joke for you: Why don't scientists trust atoms? " +
          "Because they make up everything!",
      },
      prompt: CONTINUE_CONVERSATION_CALL.request.prompt,
    },
  },
};
