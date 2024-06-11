import type { Prompt } from "$lib/bindings";

export const EDIT_CANONICAL_REF = {
  id: "c13c1e67-2de3-48de-a34c-a32079c03316",
  snippet:
    "Sure, here's a joke for you: Why don't scientists trust atoms? " +
    "Because they make up everything!",
};

export const EDIT_PROMPT: Prompt = {
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
};
