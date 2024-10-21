import ApiCallEditorComponent from "./ApiCallEditor.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import { CONTINUE_CONVERSATION_CALL } from "../[slug]/sample-calls";
import { EMOJI_CANONICAL_REF } from "./test.data";

export default {
  component: ApiCallEditorComponent,
  title: "Screens/Database/LLM Call/New",
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

export const Busy: StoryObj = Template.bind({}) as any;
Busy.args = {
  expectingResponse: true,
};
Busy.parameters = {
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

// note: this also applies to the API calls list, but it's easier to test here
export const WithEmoji: StoryObj = Template.bind({}) as any;
WithEmoji.parameters = {
  stores: {
    apiCallEditing: {
      canonicalRef: EMOJI_CANONICAL_REF,
      prompt: {
        type: "Chat",
        messages: [{ role: "System", text: "" }],
      },
    },
  },
};
