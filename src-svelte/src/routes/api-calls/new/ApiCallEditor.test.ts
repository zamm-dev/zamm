import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import ApiCallEditor, { resetNewApiCall } from "./ApiCallEditor.svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { get } from "svelte/store";
import { mockStores } from "../../../vitest-mocks/stores";

describe("API call editor", () => {
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
    resetNewApiCall();
  });

  async function addNewMessage() {
    const numMessages = screen.getAllByRole("listitem").length;
    const newMessageButton = screen.getByRole("button", { name: "+" });
    await userEvent.click(newMessageButton);
    expect(screen.getAllByRole("listitem").length).toBe(numMessages + 1);
  }

  async function setNewMessage(role: string, message: string) {
    const messageDivs = screen.getAllByRole("listitem");
    const lastMessageDiv = messageDivs[messageDivs.length - 1];

    const roleToggle = lastMessageDiv.querySelector(
      'span[aria-label="Toggle message type"]',
    );
    if (roleToggle === null) {
      throw new Error("Role toggle not found");
    }
    for (let i = 0; i < 3; i++) {
      if (roleToggle.textContent === role) {
        break;
      }
      await userEvent.click(roleToggle);
    }
    expect(roleToggle.textContent).toBe(role);

    const messageInput = lastMessageDiv.querySelector("textarea");
    if (messageInput === null) {
      throw new Error("Message input not found");
    }
    await userEvent.type(messageInput, message);
    expect(messageInput).toHaveValue(message);
  }

  test("can manually trigger API call with all roles", async () => {
    render(ApiCallEditor, {});
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-manual-conversation-recreation.yaml",
    );

    await setNewMessage(
      "System",
      "You are ZAMM, a chat program. Respond in first person.",
    );
    await addNewMessage();
    await setNewMessage("Human", "Hello, does this work?");
    await addNewMessage();
    await setNewMessage("AI", "Yes, it works. How can I assist you today?");
    await addNewMessage();
    await setNewMessage("Human", "Tell me something funny.");

    await userEvent.click(screen.getByRole("button", { name: "Submit" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(get(mockStores.page).url.pathname).toEqual(
      "/api-calls/c13c1e67-2de3-48de-a34c-a32079c03316",
    );
  });
});
