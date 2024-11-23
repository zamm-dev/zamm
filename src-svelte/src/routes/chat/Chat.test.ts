import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor } from "@testing-library/svelte";
import Chat, {
  conversation,
  lastMessageId,
  resetConversation,
} from "./Chat.svelte";
import PersistentChatView from "./PersistentChatView.svelte";
import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  type ParsedCall,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";
import { animationSpeed } from "$lib/preferences";
import type { ChatArgs } from "$lib/bindings";

describe("Chat conversation", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeAll(() => {
    vi.stubGlobal("requestAnimationFrame", (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
    window.IntersectionObserver = vi.fn(() => {
      return {
        observe: vi.fn(),
        unobserve: vi.fn(),
        disconnect: vi.fn(),
      };
    }) as unknown as typeof IntersectionObserver;
    Range.prototype.getBoundingClientRect = vi.fn(() => {
      return {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
        top: 0,
        left: 0,
        right: 10,
        bottom: 10,
        toJSON: vi.fn(),
      };
    });
    HTMLElement.prototype.animate = vi.fn().mockReturnValue({
      onfinish: null,
      cancel: vi.fn(),
    });

    animationSpeed.set(10);
  });

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    stubGlobalInvoke(tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    resetConversation();
  });

  async function sendChatMessage(
    message: string,
    expectedAiResponse: string,
    correspondingApiCallSample: string,
  ) {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(correspondingApiCallSample);
    const nextExpectedApiCall: ParsedCall =
      playback.unmatchedCalls.slice(-1)[0];
    const nextExpectedCallArgs = nextExpectedApiCall.request[1] as unknown as {
      args: ChatArgs;
    };
    const nextExpectedMessage =
      nextExpectedCallArgs.args["prompt"].slice(-1)[0];
    const nextExpectedHumanPrompt = nextExpectedMessage.text;

    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, message);
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();

    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    await waitFor(() => {
      expect(
        screen.getByText(
          new RegExp(expectedAiResponse.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
        ),
      ).toBeInTheDocument();
    });
    expect(chatInput).toHaveValue("");

    tauriInvokeMock.mockClear();
  }

  test("can start and continue a conversation", async () => {
    render(Chat, {});
    await sendChatMessage(
      "Hello, does this work?",
      "Yes, it works. How can I assist you today?",
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );
    await sendChatMessage(
      "Tell me something funny.",
      "Sure, here's a joke for you",
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
  });

  test("won't send multiple messages at once", async () => {
    render(Chat, {});
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.callPauseMs = 2_000; // this line differs from sendChatMessage
    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );
    const nextExpectedApiCall: ParsedCall =
      playback.unmatchedCalls.slice(-1)[0];
    const nextExpectedCallArgs = nextExpectedApiCall.request[1] as unknown as {
      args: ChatArgs;
    };
    const nextExpectedMessage =
      nextExpectedCallArgs.args["prompt"].slice(-1)[0];
    const nextExpectedHumanPrompt = nextExpectedMessage.text;

    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, "Hello, does this work?");
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();
    // this part differs from sendChatMessage
    await userEvent.type(chatInput, "Tell me something funny.");
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    // remember, we're intentionally delaying the first return here,
    // so the mock won't be called
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();
    expect(chatInput).toHaveValue("Tell me something funny.");
  });

  test("persists a conversation after returning to it", async () => {
    render(PersistentChatView, {});
    await sendChatMessage(
      "Hello, does this work?",
      "Yes, it works. How can I assist you today?",
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );

    await userEvent.click(screen.getByRole("button", { name: "Remount" }));
    await waitFor(() => {
      expect(screen.getByText("Hello, does this work?")).toBeInTheDocument();
    });
  });

  test("persists chat text after returning to it", async () => {
    render(PersistentChatView, {});
    const message = "testing chat text persistence";
    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, message);
    expect(chatInput).toHaveValue(message);

    await userEvent.click(screen.getByRole("button", { name: "Remount" }));
    await waitFor(() => {
      const newChatInput = screen.getByLabelText("Chat with the AI:");
      expect(newChatInput).toHaveValue(message);
    });
  });

  test("listens for updates to conversation store", async () => {
    render(Chat, {});
    expect(
      screen.queryByText("Hello, does this work?"),
    ).not.toBeInTheDocument();
    lastMessageId.set("d5ad1e49-f57f-4481-84fb-4d70ba8a7a74");
    conversation.set([
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
    ]);
    await waitFor(() => {
      expect(screen.getByText("Hello, does this work?")).toBeInTheDocument();
    });
    await sendChatMessage(
      "Tell me something funny.",
      "Sure, here's a joke for you",
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
  });
});
