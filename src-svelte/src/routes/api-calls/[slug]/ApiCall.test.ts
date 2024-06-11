import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import {
  resetConversation,
  conversation,
  lastMessageId,
} from "../../chat/Chat.svelte";
import {
  canonicalRef,
  prompt,
  getDefaultApiCall,
  resetNewApiCall,
} from "../new/ApiCallEditor.svelte";
import { EDIT_CANONICAL_REF, EDIT_PROMPT } from "../new/test.data";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
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

    mockStores.page.set({
      url: new URL("http://localhost/"),
      params: {},
    });
    resetConversation();
    resetNewApiCall();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
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
    const responseSection = await screen.findByLabelText("Response");
    const response = responseSection.querySelector("pre");
    expect(response).toHaveTextContent(
      "Sure, here's a joke for you: Why don't scientists trust atoms? " +
        "Because they make up everything!",
    );

    // check that metadata is displayed
    expectRowValue("LLM", "gpt-4 → gpt-4-0613");
    expectRowValue("Temperature", "1.00");
    expectRowValue("Tokens", "57 prompt + 22 response = 79 total tokens");

    // check that links are displayed
    const conversationSection = await screen.findByLabelText("Conversation");
    const conversationLinks = Array.from(
      conversationSection.querySelectorAll("a"),
    ).map((a) => a.href);
    expect(conversationLinks).toEqual([
      // previous
      "http://localhost:3000/api-calls/d5ad1e49-f57f-4481-84fb-4d70ba8a7a74",
      // next
      "http://localhost:3000/api-calls/0e6bcadf-2b41-43d9-b4cf-81008d4f4771",
      "http://localhost:3000/api-calls/63b5c02e-b864-4efe-a286-fbef48b152ef",
    ]);

    const variantSection = await screen.findByLabelText("Variants");
    const variantLinks = Array.from(variantSection.querySelectorAll("a")).map(
      (a) => a.href,
    );
    expect(variantLinks).toEqual([
      "http://localhost:3000/api-calls/f39a5017-89d4-45ec-bcbb-25c2bd43cfc1",
      "http://localhost:3000/api-calls/7a35a4cf-f3d9-4388-bca8-2fe6e78c9648",
    ]);
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
    expect(get(lastMessageId)).toBeUndefined();
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
    expect(get(lastMessageId)).toEqual("c13c1e67-2de3-48de-a34c-a32079c03316");
    expect(get(mockStores.page).url.pathname).toEqual("/chat");
  });

  test("can edit API call", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_api_call-continue-conversation.yaml",
    );
    render(ApiCall, { id: "c13c1e67-2de3-48de-a34c-a32079c03316" });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    await waitFor(() => {
      screen.getByText("Hello, does this work?");
    });
    expect(get(canonicalRef)).toBeUndefined();
    expect(get(prompt)).toEqual(getDefaultApiCall());
    expect(get(mockStores.page).url.pathname).toEqual("/");

    const editButton = await waitFor(() => screen.getByText("Edit API call"));
    userEvent.click(editButton);
    await waitFor(() => {
      expect(get(prompt)).toEqual(EDIT_PROMPT);
    });
    expect(get(canonicalRef)).toEqual(EDIT_CANONICAL_REF);
    expect(get(mockStores.page).url.pathname).toEqual("/api-calls/new/");
  });
});
