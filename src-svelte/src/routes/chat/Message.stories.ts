import Message from "./Message.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Message,
  title: "Screens/Chat/Message",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Message,
  props: args,
});

export const Human: StoryObj = Template.bind({}) as any;
Human.args = {
  message: {
    role: "Human",
    text: "Hello, does this work?",
  },
};
Human.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const AI: StoryObj = Template.bind({}) as any;
AI.args = {
  message: {
    role: "AI",
    text:
      "Hello! I'm ZAMM, a chat program. I'm here to assist you. " +
      "What can I help you with today?",
  },
};
AI.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const AIMultiline: StoryObj = Template.bind({}) as any;
AIMultiline.args = {
  message: {
    role: "AI",
    text:
      "Sure, here's a light-hearted joke for you:\n\n" +
      "Why don't scientists trust atoms?\n\n" +
      "Because they make up everything!",
  },
};
AIMultiline.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

const humanCodeMessage = {
  role: "Human",
  text:
    "This is some Python code:\n\n" +
    "```python\n" +
    "def hello_world():\n" +
    "    print('Hello, world!')\n" +
    "```\n\n" +
    "Convert it to Rust",
};

const aiCodeMessage = {
  role: "AI",
  text:
    "Here's how the Python code you provided would look in Rust:\n\n" +
    '```rust\nfn main() {\n    println!("Hello, world!");\n}\n```',
};

export const HumanCode: StoryObj = Template.bind({}) as any;
HumanCode.args = {
  message: humanCodeMessage,
};
HumanCode.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const HighlightedHumanCode: StoryObj = Template.bind({}) as any;
HighlightedHumanCode.args = {
  message: humanCodeMessage,
  forceHighlight: true,
};
HighlightedHumanCode.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const AICode: StoryObj = Template.bind({}) as any;
AICode.args = {
  message: aiCodeMessage,
};
AICode.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const HighlightedAiCode: StoryObj = Template.bind({}) as any;
HighlightedAiCode.args = {
  message: aiCodeMessage,
  forceHighlight: true,
};
HighlightedAiCode.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};
