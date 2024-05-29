import PromptComponent from "./Prompt.svelte";
import type { StoryObj } from "@storybook/svelte";
import { CONTINUE_CONVERSATION_CALL } from "./sample-calls";

export default {
  component: PromptComponent,
  title: "Screens/LLM Call/Individual/Prompt",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: PromptComponent,
  props: args,
});

export const Uneditable: StoryObj = Template.bind({}) as any;
Uneditable.args = {
  prompt: CONTINUE_CONVERSATION_CALL.request.prompt,
};

export const Editable: StoryObj = Template.bind({}) as any;
Editable.args = {
  editable: true,
  prompt: CONTINUE_CONVERSATION_CALL.request.prompt,
};
