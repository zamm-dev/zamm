import type { ChatMessage } from "$lib/bindings";

export const shortConversation: ChatMessage[] = [
  {
    role: "System",
    text: "You are ZAMM, a chat program. Respond in first person.",
  },
  {
    role: "Human",
    text: "Hello, does this work?",
  },
];

export const conversation: ChatMessage[] = [
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
