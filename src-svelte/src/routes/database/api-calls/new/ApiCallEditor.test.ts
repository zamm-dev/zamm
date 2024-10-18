import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen, waitFor } from "@testing-library/svelte";
import ApiCallEditor, {
  canonicalRef,
  prompt,
  provider,
  llm,
  resetNewApiCall,
} from "./ApiCallEditor.svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { get } from "svelte/store";
import { mockStores } from "../../../../vitest-mocks/stores";
import { EDIT_CANONICAL_REF, EDIT_PROMPT } from "./test.data";
import PersistentApiCallEditorView from "./PersistentApiCallEditorView.svelte";

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

  async function getLastMessageComponents() {
    const messageDivs = screen.getAllByRole("listitem");
    const lastMessageDiv = messageDivs[messageDivs.length - 1];

    const roleToggle = lastMessageDiv.querySelector(
      'span[aria-label="Toggle message type"]',
    );
    if (roleToggle === null) {
      throw new Error("Role toggle not found");
    }

    const messageInput = lastMessageDiv.querySelector("textarea");
    if (messageInput === null) {
      throw new Error("Message input not found");
    }

    return {
      lastMessageDiv,
      roleToggle,
      messageInput,
    };
  }

  async function setNewMessage(role: string, message: string) {
    const { roleToggle, messageInput } = await getLastMessageComponents();

    for (let i = 0; i < 3; i++) {
      if (roleToggle.textContent === role) {
        break;
      }
      await userEvent.click(roleToggle);
    }
    expect(roleToggle.textContent).toBe(role);

    if (messageInput.value !== "") {
      await userEvent.clear(messageInput);
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
      "/database/api-calls/c13c1e67-2de3-48de-a34c-a32079c03316",
    );
  });

  test("can edit an existing API call", async () => {
    canonicalRef.set(EDIT_CANONICAL_REF);
    prompt.set(EDIT_PROMPT);
    render(ApiCallEditor, {});
    const originalApiCallLabel = screen.getByText("Original API call:");
    const originalApiCallLink = originalApiCallLabel.nextElementSibling;
    if (originalApiCallLink === null) {
      throw new Error("Original API call link not found");
    }
    expect(originalApiCallLink).toHaveTextContent(EDIT_CANONICAL_REF.snippet);
    expect(originalApiCallLink).toHaveAttribute(
      "href",
      "/database/api-calls/c13c1e67-2de3-48de-a34c-a32079c03316",
    );

    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-edit-conversation.yaml",
    );
    await setNewMessage("Human", "Tell me a funny joke.");

    await userEvent.click(screen.getByRole("button", { name: "Submit" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(get(mockStores.page).url.pathname).toEqual(
      "/database/api-calls/f39a5017-89d4-45ec-bcbb-25c2bd43cfc1",
    );
  });

  test("can make a call to Llama 3", async () => {
    render(ApiCallEditor, {});
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(
      "../src-tauri/api/sample-calls/chat-start-conversation-ollama.yaml",
    );

    await setNewMessage(
      "System",
      "You are ZAMM, a chat program. Respond in first person.",
    );
    await addNewMessage();
    await setNewMessage("Human", "Hello, does this work?");

    const providerSelect = screen.getByRole("combobox", { name: "Provider:" });
    await userEvent.selectOptions(providerSelect, "Ollama");

    await userEvent.click(screen.getByRole("button", { name: "Submit" }));
    expect(tauriInvokeMock).toHaveBeenCalledTimes(1);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(get(mockStores.page).url.pathname).toEqual(
      "/database/api-calls/506e2d1f-549c-45cc-ad65-57a0741f06ee",
    );

    // test that the provider is preserved
    expect(get(provider)).toEqual("Ollama");
    expect(get(llm)).toEqual("llama3:8b");
  });

  test("will update model list when provider changes", async () => {
    render(ApiCallEditor, {});

    const providerSelect = screen.getByRole("combobox", { name: "Provider:" });
    const modelSelect = screen.getByRole("combobox", { name: "Model:" });
    expect(providerSelect).toHaveValue("OpenAI");
    expect(modelSelect).toHaveValue("gpt-4");

    await userEvent.selectOptions(providerSelect, "Ollama");
    expect(providerSelect).toHaveValue("Ollama");
    expect(modelSelect).toHaveValue("llama3:8b");

    // test that model can change without affecting provider
    await userEvent.selectOptions(modelSelect, "gemma2:9b");
    expect(providerSelect).toHaveValue("Ollama");
    expect(modelSelect).toHaveValue("gemma2:9b");

    // test that we can switch things back
    await userEvent.selectOptions(providerSelect, "OpenAI");
    expect(providerSelect).toHaveValue("OpenAI");
    expect(modelSelect).toHaveValue("gpt-4");
  });

  test("will preserve all settings on page change", async () => {
    render(PersistentApiCallEditorView, {});

    // get controls
    const { roleToggle, messageInput } = await getLastMessageComponents();
    const providerSelect = screen.getByRole("combobox", { name: "Provider:" });
    const modelSelect = screen.getByRole("combobox", { name: "Model:" });
    // make changes
    await userEvent.selectOptions(providerSelect, "Ollama");
    await userEvent.selectOptions(modelSelect, "gemma2:9b");
    await userEvent.click(roleToggle);
    await userEvent.type(messageInput, "Hello, does this work?");
    // affirm changes
    expect(roleToggle.textContent).toBe("Human");
    expect(messageInput).toHaveValue("Hello, does this work?");
    expect(providerSelect).toHaveValue("Ollama");
    expect(modelSelect).toHaveValue("gemma2:9b");

    await userEvent.click(screen.getByRole("button", { name: "Remount" }));
    await waitFor(async () => {
      const {
        roleToggle: refreshedRoleToggle,
        messageInput: refreshedMessageInput,
      } = await getLastMessageComponents();
      const refreshedProviderSelect = screen.getByRole("combobox", {
        name: "Provider:",
      });
      const refreshedModelSelect = screen.getByRole("combobox", {
        name: "Model:",
      });
      expect(refreshedRoleToggle.textContent).toBe("Human");
      expect(refreshedMessageInput).toHaveValue("Hello, does this work?");
      expect(refreshedProviderSelect).toHaveValue("Ollama");
      expect(refreshedModelSelect).toHaveValue("gemma2:9b");
    });
  });
});
