import ChatComponent from "./Chat.svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import type { ChatMessage } from "$lib/bindings";

export default {
  component: ChatComponent,
  title: "Screens/Chat/Conversation",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockFullPageLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: ChatComponent,
  props: args,
});

export const Empty: StoryObj = Template.bind({}) as any;
Empty.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

const shortConversation: ChatMessage[] = [
  {
    role: "System",
    text: "You are ZAMM, a chat program. Respond in first person.",
  },
  {
    role: "Human",
    text: "Hello, does this work?",
  },
];

const conversation: ChatMessage[] = [
  {
    role: "System",
    text: "You are ZAMM, a chat program. Respond in first person.",
  },
  {
    role: "Human",
    text: "Hello, does this work?",
  },
  {
    role: "AI",
    text:
      "Hello! I'm ZAMM, a chat program. I'm here to assist you. " +
      "What can I help you with today?",
  },
  {
    role: "Human",
    text: "Tell me something really funny, like really funny. Make me laugh hard.",
  },
  {
    role: "AI",
    text:
      "Sure, here's a light-hearted joke for you:\n\n" +
      "Why don't scientists trust atoms?\n\n" +
      "Because they make up everything!",
  },
  {
    role: "Human",
    text:
      "This is some Python code:\n\n" +
      "```python\n" +
      "def hello_world():\n" +
      "    print('Hello, world!')\n" +
      "```\n\n" +
      "Convert it to Rust",
  },
  {
    role: "AI",
    text:
      "Here's how the Python code you provided would look in Rust:\n\n" +
      '```rust\nfn main() {\n    println!("Hello, world!");\n}\n```',
  },
];

export const NotEmpty: StoryObj = Template.bind({}) as any;
NotEmpty.args = {
  conversation,
};
NotEmpty.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const MultilineChat: StoryObj = Template.bind({}) as any;
MultilineChat.args = {
  conversation,
  initialMessage:
    "This is what happens when the user types in so much text, " +
    "it wraps around and turns the text input area into a multiline input. " +
    "The send button's height should grow in line with the overall text area height.",
};
MultilineChat.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const ExtraLongInput: StoryObj = Template.bind({}) as any;
ExtraLongInput.args = {
  conversation,
  initialMessage: `Hey, I have this definition for a book object:

\`\`\`python
class Book:
  def __init__(self, title, author, pages):
      self.title = title
      self.author = author
      self.pages = pages

  def book_info(self):
      return f"'{self.title}' by {self.author} has {self.pages} pages."

  def is_long(self):
      return self.pages > 200
\`\`\`

Do you have any code comments for me?`,
};
ExtraLongInput.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const BottomScrollIndicator: StoryObj = Template.bind({}) as any;
BottomScrollIndicator.args = {
  conversation,
  showMostRecentMessage: false,
};
BottomScrollIndicator.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const FullMessageWidth: StoryObj = Template.bind({}) as any;
FullMessageWidth.args = {
  conversation,
};
FullMessageWidth.parameters = {
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const TypingIndicator: StoryObj = Template.bind({}) as any;
TypingIndicator.args = {
  conversation: shortConversation,
  expectingResponse: true,
};
TypingIndicator.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const TypingIndicatorStatic: StoryObj = Template.bind({}) as any;
TypingIndicatorStatic.args = {
  conversation: shortConversation,
  expectingResponse: true,
};
TypingIndicatorStatic.parameters = {
  preferences: {
    animationsOn: false,
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.decorators = [
  (story: StoryFn) => {
    return {
      Component: MockPageTransitions,
      slot: story,
    };
  },
];
