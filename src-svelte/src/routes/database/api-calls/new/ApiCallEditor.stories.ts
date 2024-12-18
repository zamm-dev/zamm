import ApiCallEditorComponent from "./ApiCallEditor.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import { CONTINUE_CONVERSATION_PROMPT } from "../[slug]/sample-calls";
import { EDIT_CANONICAL_REF, EMOJI_CANONICAL_REF } from "./test.data";
import { MockAppLayoutDecorator } from "$lib/__mocks__/decorators";

export default {
  component: ApiCallEditorComponent,
  title: "Screens/Database/LLM Call/New",
  argTypes: {},
  decorators: [SvelteStoresDecorator, MockAppLayoutDecorator],
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
      canonicalRef: EDIT_CANONICAL_REF,
      prompt: CONTINUE_CONVERSATION_PROMPT,
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
      canonicalRef: EDIT_CANONICAL_REF,
      prompt: CONTINUE_CONVERSATION_PROMPT,
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
