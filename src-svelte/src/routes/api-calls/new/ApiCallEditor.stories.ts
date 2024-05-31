import ApiCallEditorComponent from "./ApiCallEditor.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ApiCallEditorComponent,
  title: "Screens/LLM Call/New",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ApiCallEditorComponent,
  props: args,
});

export const Blank: StoryObj = Template.bind({}) as any;
