import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor } from "@testing-library/svelte";
import Chat, { conversation, resetConversation } from "./Chat.svelte";
import PersistentChatView from "./PersistentChatView.svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback, type ParsedCall } from "$lib/sample-call-testing";
import { animationSpeed } from "$lib/preferences";
import type { ChatMessage, LightweightLlmCall } from "$lib/bindings";

describe("Chat conversation", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeAll(() => {
    animationSpeed.set(10);
  });

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );

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
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    resetConversation();
  });

  async function sendChatMessage(
    message: string,
    correspondingApiCallSample: string,
  ) {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(correspondingApiCallSample);
    const nextExpectedApiCall: ParsedCall =
      playback.unmatchedCalls.slice(-1)[0];
    const nextExpectedCallArgs = nextExpectedApiCall.request[1] as Record<
      string,
      any
    >;
    const nextExpectedMessage = nextExpectedCallArgs["prompt"].slice(
      -1,
    )[0] as ChatMessage;
    const nextExpectedHumanPrompt = nextExpectedMessage.text;

    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, message);
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();

    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    const lastResult: LightweightLlmCall =
      tauriInvokeMock.mock.results[0].value;
    const aiResponse = lastResult.response_message.text;
    const lastSentence = aiResponse.split("\n").slice(-1)[0];
    await waitFor(() => {
      expect(
        screen.getByText(
          new RegExp(lastSentence.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
        ),
      ).toBeInTheDocument();
    });

    tauriInvokeMock.mockClear();
  }

  test("can start and continue a conversation", async () => {
    render(Chat, {});
    await sendChatMessage(
      "Hello, does this work?",
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );
    await sendChatMessage(
      "Tell me something funny.",
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
  });

  test("won't send multiple messages at once", async () => {
    render(Chat, {});
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.callPauseMs = 1_000; // this line differs from sendChatMessage
    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );
    const nextExpectedApiCall: ParsedCall =
      playback.unmatchedCalls.slice(-1)[0];
    const nextExpectedCallArgs = nextExpectedApiCall.request[1] as Record<
      string,
      any
    >;
    const nextExpectedMessage = nextExpectedCallArgs["prompt"].slice(
      -1,
    )[0] as ChatMessage;
    const nextExpectedHumanPrompt = nextExpectedMessage.text;

    const chatInput = screen.getByLabelText("Chat with the AI:");
    expect(chatInput).toHaveValue("");
    await userEvent.type(chatInput, "Hello, does this work?");
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(screen.getByText(nextExpectedHumanPrompt)).toBeInTheDocument();
    // this part differs from sendChatMessage
    await waitFor(() => {
      expect(
        screen.getByText("Yes, it works. How can I assist you today?"),
      ).toBeInTheDocument();
    });

    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
    await userEvent.type(chatInput, "Tell me something funny.");
    await userEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(2);
    expect(screen.getByText("Tell me something funny.")).toBeInTheDocument();
    await waitFor(() => {
      expect(
        screen.getByText(/Because they make up everything/),
      ).toBeInTheDocument();
    });
  });

  test("persists a conversation after returning to it", async () => {
    render(PersistentChatView, {});
    await sendChatMessage(
      "Hello, does this work?",
      "../src-tauri/api/sample-calls/chat-start-conversation.yaml",
    );

    await userEvent.click(screen.getByRole("button", { name: "Remount" }));
    await waitFor(() => {
      expect(screen.getByText("Hello, does this work?")).toBeInTheDocument();
    });
  });

  test("listens for updates to conversation store", async () => {
    render(Chat, {});
    expect(
      screen.queryByText("Hello, does this work?"),
    ).not.toBeInTheDocument();
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
      "../src-tauri/api/sample-calls/chat-continue-conversation.yaml",
    );
  });
});
