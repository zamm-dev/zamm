import MessageComponent from "./Message.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: MessageComponent,
  title: "Layout/Snackbar/Message",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: MessageComponent,
  props: args,
});

export const Error: StoryObj = Template.bind({}) as any;
Error.args = {
  message: "Something is wrong.",
  messageType: "error",
  dismiss: () => {
    console.log("Dismiss button clicked.");
  },
};
Error.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const Info: StoryObj = Template.bind({}) as any;
Info.args = {
  message: "Something is known.",
  messageType: "info",
  dismiss: () => {
    console.log("Dismiss button clicked.");
  },
};
Info.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};
