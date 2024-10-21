import { expect, test } from "vitest";
import "@testing-library/jest-dom";

import { parseImportData } from "./ApiCallImport.svelte";

/* eslint-disable max-len */

const SAMPLE_LM_STUDIO_EXPORT = `[
  {
    "role": "system",
    "content": "You are a helpful, smart, kind, and efficient AI assistant. You always fulfill the user's requests to the best of your ability."
  },
  {
    "role": "user",
    "content": "Hey there"
  },
  {
    "role": "assistant",
    "content": "Hello! It's great to chat with you. Is there something I can help you with or would you like to talk about something in particular? I'm all ears (or rather, all text)!"
  },
  {
    "role": "user",
    "content": "I'm just trying you out for the first time! I'm curious what your personality is like"
  },
  {
    "role": "assistant",
    "content": "Well, let me tell you a little bit about myself!\\n\\nI'm a helpful AI assistant, and my goal is to assist and provide information to users in a friendly and efficient manner. I'm designed to be kind, smart, and efficient.\\n\\nAs for my personality, I'd say I'm a bit of a chatty AI! I love to engage in conversations and answer questions to the best of my ability. I'm always happy to help with any topic or subject you're interested in learning more about.\\n\\nI'm also very patient and understanding. I know that sometimes users might have different levels of knowledge or experience, so I try to explain things in a way that's easy to understand.\\n\\nBut don't worry, I won't overwhelm you with too much information! I just want to be a helpful resource for you whenever you need it.\\n\\nSo, how do you like me so far?"
  }
]`;

const SAMPLE_OLLAMA_EXPORT = `>>> Hey there
Hi! It's nice to meet you. Is there something I can help you with or would 
you like to chat?

>>> Hi, I'm trying to see how I can save conversations from you in a different a
... pp. I'm currently running you in a CLI called Ollama, and I need to copy and
...  parse the terminal window content.
I'm happy to help you with that.

Since you're using Ollama, which is a command-line interface, we can work 
together to figure out how to save our conversation. To get started, could 
you tell me what format or app you'd like to export our conversation to 
(e.g., text file, JSON, Markdown, etc.)? Additionally, are there any 
specific requirements or constraints I should keep in mind while 
formatting the output?

Also, just to confirm: you want to copy and parse the terminal window 
content, which means you'll need to capture the entire conversation 
including my responses. Is that correct?

`;

describe("API call import", () => {
  test("parses LM Studio exported conversation successfully", async () => {
    expect(parseImportData(SAMPLE_LM_STUDIO_EXPORT)).toEqual([
      {
        role: "System",
        text: "You are a helpful, smart, kind, and efficient AI assistant. You always fulfill the user's requests to the best of your ability.",
      },
      {
        role: "Human",
        text: "Hey there",
      },
      {
        role: "AI",
        text: "Hello! It's great to chat with you. Is there something I can help you with or would you like to talk about something in particular? I'm all ears (or rather, all text)!",
      },
      {
        role: "Human",
        text: "I'm just trying you out for the first time! I'm curious what your personality is like",
      },
      {
        role: "AI",
        text: "Well, let me tell you a little bit about myself!\n\nI'm a helpful AI assistant, and my goal is to assist and provide information to users in a friendly and efficient manner. I'm designed to be kind, smart, and efficient.\n\nAs for my personality, I'd say I'm a bit of a chatty AI! I love to engage in conversations and answer questions to the best of my ability. I'm always happy to help with any topic or subject you're interested in learning more about.\n\nI'm also very patient and understanding. I know that sometimes users might have different levels of knowledge or experience, so I try to explain things in a way that's easy to understand.\n\nBut don't worry, I won't overwhelm you with too much information! I just want to be a helpful resource for you whenever you need it.\n\nSo, how do you like me so far?",
      },
    ]);
  });

  test("parses Ollama copy-pasted conversation successfully", async () => {
    expect(parseImportData(SAMPLE_OLLAMA_EXPORT)).toEqual([
      {
        role: "Human",
        text: "Hey there",
      },
      {
        role: "AI",
        text: "Hi! It's nice to meet you. Is there something I can help you with or would you like to chat?",
      },
      {
        role: "Human",
        text: "Hi, I'm trying to see how I can save conversations from you in a different app. I'm currently running you in a CLI called Ollama, and I need to copy and parse the terminal window content.",
      },
      {
        role: "AI",
        text: "I'm happy to help you with that.\n\nSince you're using Ollama, which is a command-line interface, we can work together to figure out how to save our conversation. To get started, could you tell me what format or app you'd like to export our conversation to (e.g., text file, JSON, Markdown, etc.)? Additionally, are there any specific requirements or constraints I should keep in mind while formatting the output?\n\nAlso, just to confirm: you want to copy and parse the terminal window content, which means you'll need to capture the entire conversation including my responses. Is that correct?",
      },
    ]);
  });
});
