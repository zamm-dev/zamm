import ChatComponent from "./Chat.svelte";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import { conversation, shortConversation } from "./Chat.mock-data";

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

export const NotEmpty: StoryObj = Template.bind({}) as any;
NotEmpty.parameters = {
  stores: {
    chat: {
      conversation,
    },
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const MultilineChat: StoryObj = Template.bind({}) as any;
MultilineChat.parameters = {
  stores: {
    chat: {
      conversation,
      nextChatMessage:
        "This is what happens when the user types in so much text, " +
        "it wraps around and turns the text input area into a multiline input. " +
        "The send button's height should grow in line with the overall text area " +
        "height.",
    },
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const ExtraLongInput: StoryObj = Template.bind({}) as any;
ExtraLongInput.parameters = {
  stores: {
    chat: {
      conversation,
      nextChatMessage: `Hey, I have this definition for a book object:

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
    },
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const BottomScrollIndicator: StoryObj = Template.bind({}) as any;
BottomScrollIndicator.args = {
  showMostRecentMessage: false,
};
BottomScrollIndicator.parameters = {
  stores: {
    chat: {
      conversation,
    },
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const FullMessageWidth: StoryObj = Template.bind({}) as any;
FullMessageWidth.parameters = {
  stores: {
    chat: {
      conversation,
    },
  },
  viewport: {
    defaultViewport: "mobile1",
  },
};

export const TypingIndicator: StoryObj = Template.bind({}) as any;
TypingIndicator.args = {
  expectingResponse: true,
};
TypingIndicator.parameters = {
  stores: {
    chat: {
      conversation: shortConversation,
    },
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const TypingIndicatorStatic: StoryObj = Template.bind({}) as any;
TypingIndicatorStatic.args = {
  expectingResponse: true,
};
TypingIndicatorStatic.parameters = {
  stores: {
    chat: {
      conversation: shortConversation,
    },
  },
  preferences: {
    animationsOn: false,
  },
  viewport: {
    defaultViewport: "smallTablet",
  },
};
