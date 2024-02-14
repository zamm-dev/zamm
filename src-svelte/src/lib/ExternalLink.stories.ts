import ExternalLinkView from "./ExternalLinkView.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: ExternalLinkView,
  title: "Reusable/External Link",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: ExternalLinkView,
  props: args,
});

export const ExternalLink: StoryObj = Template.bind({}) as any;
