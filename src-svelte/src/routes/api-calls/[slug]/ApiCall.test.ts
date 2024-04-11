import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { resetConversation } from "../../chat/Chat.svelte";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { conversation } from "../../chat/Chat.svelte";
import ApiCall from "./ApiCall.svelte";
import { get } from "svelte/store";
import { mockStores } from "../../../vitest-mocks/stores";

describe("Individual API call", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
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

  function expectRowValue(key: string, expectedValue: string) {
    const keyRegex = new RegExp(key);
    const row = screen.getByRole("row", { name: keyRegex });
    const rowValueCell = within(row).getAllByRole("cell")[1];
    expect(rowValueCell).toHaveTextContent(expectedValue);
  }

  test("can load API call with correct details", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_api_call-continue-conversation.yaml",
    );
    render(ApiCall, { id: "c13c1e67-2de3-48de-a34c-a32079c03316" });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);

    // check that system message is displayed
    await waitFor(() => {
      expect(
        screen.getByText(
          "You are ZAMM, a chat program. Respond in first person.",
        ),
      ).toBeInTheDocument();
    });
    // check that human message is displayed
    expect(screen.getByText("Hello, does this work?")).toBeInTheDocument();
    // check that AI message is displayed
    expect(
      screen.getByText(
        "Sure, here's a joke for you: " +
          "Why don't scientists trust atoms? " +
          "Because they make up everything!",
      ),
    ).toBeInTheDocument();
    // check that metadata is displayed
    expectRowValue("LLM", "gpt-4 → gpt-4-0613");
    expectRowValue("Temperature", "1.00");
    expectRowValue("Tokens", "57 prompt + 22 response = 79 total tokens");
  });

  test("can restore chat conversation", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_api_call-continue-conversation.yaml",
    );
    render(ApiCall, { id: "c13c1e67-2de3-48de-a34c-a32079c03316" });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    await waitFor(() => {
      screen.getByText("Hello, does this work?");
    });
    expect(get(conversation)).toEqual([
      {
        role: "System",
        text: "You are ZAMM, a chat program. Respond in first person.",
      },
    ]);
    expect(get(mockStores.page).url.pathname).toEqual("/");

    const restoreButton = await waitFor(() =>
      screen.getByText("Restore conversation"),
    );
    userEvent.click(restoreButton);
    await waitFor(() => {
      expect(get(conversation)).toEqual([
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
        {
          role: "AI",
          text:
            "Sure, here's a joke for you: Why don't scientists trust atoms? " +
            "Because they make up everything!",
        },
      ]);
    });
    expect(get(mockStores.page).url.pathname).toEqual("/chat");
  });
});
