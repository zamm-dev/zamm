import PersistentApiCallEditor from "./PersistentApiCallEditorView.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";

export default {
  component: PersistentApiCallEditor,
  title: "Screens/Database/LLM Call/New",
  argTypes: {},
  decorators: [SvelteStoresDecorator],
};

const Template = ({ ...args }) => ({
  Component: PersistentApiCallEditor,
  props: args,
});

export const Remountable: StoryObj = Template.bind({}) as any;
