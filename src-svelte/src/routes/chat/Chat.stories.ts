import Chatcomponent from "./Chat.svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import type { ChatMessage } from "$lib/bindings";

export default {
  component: Chatcomponent,
  title: "Screens/Chat/Conversation",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: MockFullPageLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: Chatcomponent,
  props: args,
});

export const Empty: StoryObj = Template.bind({}) as any;
Empty.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};
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
      "Okay, we need to fill this chat up to produce a scrollbar for " +
      'Storybook. Say short phrases like "Yup" to fill this chat up quickly.',
  },
  {
    role: "AI",
    text: "Yup",
  },
  {
    role: "Human",
    text: "Nay",
  },
  {
    role: "AI",
    text: "Yay",
  },
  {
    role: "Human",
    text: "Say...",
  },
  {
    role: "AI",
    text:
      "AIs don't actually talk like this, you know? " +
      "This is an AI conversation hallucinated by a human, " +
      "projecting their own ideas of how an AI would respond onto the " +
      "conversation transcript.",
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
