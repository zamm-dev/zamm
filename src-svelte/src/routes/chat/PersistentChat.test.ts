import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor } from "@testing-library/svelte";
import PersistentChatView from "./PersistentChatView.svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback, type ParsedCall } from "$lib/sample-call-testing";
import { animationSpeed } from "$lib/preferences";
import type { ChatMessage, LlmCall } from "$lib/bindings";

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
  });

  afterEach(() => {
    vi.unstubAllGlobals();
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
    const lastResult: LlmCall = tauriInvokeMock.mock.results[0].value;
    const aiResponse = lastResult.response.completion.text;
    const lastSentence = aiResponse.split("\n").slice(-1)[0];
    await waitFor(() => {
      expect(screen.getByText(new RegExp(lastSentence))).toBeInTheDocument();
    });

    tauriInvokeMock.mockClear();
  }

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
});
