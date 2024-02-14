import MessageComponent from "./Message.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: MessageComponent,
  title: "Layout/Snackbar",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: MessageComponent,
  props: args,
});

export const Message: StoryObj = Template.bind({}) as any;
Message.args = {
  message: "Something is wrong.",
  dismiss: () => {
    console.log("Dismiss button clicked.");
  },
};
Message.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};
