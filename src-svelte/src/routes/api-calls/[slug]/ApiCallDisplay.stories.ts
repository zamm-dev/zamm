import ApiCallComponent from "./ApiCallDisplay.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";

const CONTINUE_CONVERSATION_CALL = {
  id: "c13c1e67-2de3-48de-a34c-a32079c03316",
  timestamp: "2024-01-16T09:50:19.738093890",
  llm: {
    name: "gpt-4-0613",
    requested: "gpt-4",
    provider: "OpenAI",
  },
  request: {
    prompt: {
      type: "Chat",
      messages: [
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
          text: "Yes, it works. How can I assist you today?",
        },
        {
          role: "Human",
          text: "Tell me something funny.",
        },
      ],
    },
    temperature: 1.0,
  },
  response: {
    completion: {
      role: "AI",
      text:
        "Sure, here's a joke for you: Why don't scientists trust atoms? " +
        "Because they make up everything!",
    },
  },
  tokens: {
    prompt: 57,
    response: 22,
    total: 79,
  },
};

const KHMER_CALL = {
  id: "92665f19-be8c-48f2-b483-07f1d9b97370",
  timestamp: "2024-04-10T07:22:12.752276900",
  llm: {
    name: "gpt-4-0613",
    requested: "gpt-4",
    provider: "OpenAI",
  },
  request: {
    prompt: {
      type: "Chat",
      messages: [
        {
          role: "System",
          text: "You are ZAMM, a chat program. Respond in first person.",
        },
        {
          role: "Human",
          text: "Hello, សួស្ដី, what languages do you speak? ចេះខ្មែរអត់?",
        },
      ],
    },
    temperature: 1.0,
  },
  response: {
    completion: {
      role: "AI",
      text:
        "Hello! I am capable of understanding and responding in many languages, " +
        "including Khmer. So, yes, ខ្មែរបាទ/ចាស, I understand Khmer. " +
        "How can I assist you today?",
    },
  },
  tokens: {
    prompt: 68,
    response: 52,
    total: 120,
  },
};

export default {
  component: ApiCallComponent,
  title: "Screens/LLM Call/Individual",
  argTypes: {},
  decorators: [TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiCallComponent,
  props: args,
});

export const Narrow: StoryObj = Template.bind({}) as any;
Narrow.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Narrow.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};

export const Wide: StoryObj = Template.bind({}) as any;
Wide.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Wide.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const Khmer: StoryObj = Template.bind({}) as any;
Khmer.args = {
  apiCall: KHMER_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Khmer.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};
