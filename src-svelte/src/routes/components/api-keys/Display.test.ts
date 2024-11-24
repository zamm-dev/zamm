import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import Snackbar, { clearAllMessages } from "$lib/snackbar/Snackbar.svelte";
import ApiKeysDisplay from "./Display.svelte";
import { within, waitFor } from "@testing-library/dom";
import userEvent from "@testing-library/user-event";
import { systemInfo, NullSystemInfo } from "$lib/system-info";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";
import { animationSpeed, animationsOn } from "$lib/preferences";

describe("API Keys Display", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeAll(() => {
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

    vi.stubGlobal("requestAnimationFrame", (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
    clearAllMessages();
    animationsOn.set(false);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  function getOpenAiStatus() {
    const openAiRow = screen.getByRole("row", { name: /OpenAI/ });
    const openAiKeyCell = within(openAiRow).getAllByRole("cell")[1];
    return openAiKeyCell.textContent;
  }

  async function checkSampleCall(filename: string, expected_display: string) {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(filename);

    render(ApiKeysDisplay, {});
    await waitFor(() =>
      expect(screen.getByRole("row", { name: /OpenAI/ })).toBeInTheDocument(),
    );
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(getOpenAiStatus()).toBe(expected_display);
  }

  async function toggleOpenAIForm() {
    const openAiCell = screen.getByRole("cell", { name: "OpenAI" });
    await userEvent.click(openAiCell);
  }

  test("loading by default", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
    );

    render(ApiKeysDisplay, {});

    const status = screen.getByRole("status");
    expect(status).toHaveTextContent(/^...loading$/);
  });

  test("no API key set", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
  });

  test("some API key set", async () => {
    systemInfo.set({
      ...NullSystemInfo,
      shell_init_file: "/home/rando/.zshrc",
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    await toggleOpenAIForm();
    const apiKeyInput = screen.getByLabelText("API key:");
    expect(apiKeyInput).toHaveValue("0p3n41-4p1-k3y");
    const saveFileInput = screen.getByLabelText("Export from:");
    expect(saveFileInput).toHaveValue("/home/rando/.zshrc");
  });

  test("API key error", async () => {
    const errorMessage = "Testing error message";
    playback.addCalls({
      request: ["get_api_keys"],
      response: errorMessage,
      succeeded: false,
    });

    render(ApiKeysDisplay, {});
    await waitFor(() =>
      expect(screen.getByRole("row", { name: /OpenAI/ })).toBeInTheDocument(),
    );
    expect(tauriInvokeMock).toHaveBeenLastCalledWith(
      "get_api_keys",
      {},
      undefined,
    );

    render(Snackbar, {});
    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(errorMessage);
  });

  test("shows link to API key", async () => {
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
      "Active",
    );

    await toggleOpenAIForm();
    const apiKeyLink = screen.getByRole("link", { name: "here" });
    expect(apiKeyLink).toHaveAttribute(
      "href",
      "https://platform.openai.com/api-keys",
    );
  });

  test("can edit API key", async () => {
    systemInfo.set({
      ...NullSystemInfo,
      shell_init_file: ".bashrc",
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-existing-no-newline.yaml",
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );

    await toggleOpenAIForm();
    const apiKeyInput = screen.getByLabelText("API key:");
    expect(apiKeyInput).toHaveValue("");
    await userEvent.type(apiKeyInput, "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));
    await waitFor(() => expect(apiKeyInput).not.toBeInTheDocument());
  });

  test("can submit with custom file", async () => {
    const defaultInitFile = "/home/rando/.bashrc";
    systemInfo.set({
      ...NullSystemInfo,
      shell_init_file: defaultInitFile,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-nested-folder.yaml",
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );

    await toggleOpenAIForm();
    const fileInput = screen.getByLabelText("Export from:");
    await userEvent.clear(fileInput);
    expect(fileInput).toHaveValue("");
    await userEvent.type(fileInput, "folder/.bashrc");
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    await waitFor(() => expect(tauriInvokeMock).toHaveResolvedTimes(2));
  });

  test("can submit with no file", async () => {
    const defaultInitFile = "/home/rando/.bashrc";
    systemInfo.set({
      ...NullSystemInfo,
      shell_init_file: defaultInitFile,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-no-disk-write.yaml",
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );

    await toggleOpenAIForm();
    await userEvent.click(
      screen.getByLabelText("Export as environment variable?"),
    );
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    await waitFor(() => expect(tauriInvokeMock).toHaveResolvedTimes(2));
  });

  test("can submit with invalid file", async () => {
    systemInfo.set({
      ...NullSystemInfo,
    });
    await checkSampleCall(
      "../src-tauri/api/sample-calls/get_api_keys-empty.yaml",
      "Inactive",
    );
    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_api_key-invalid-filename.yaml",
      "../src-tauri/api/sample-calls/get_api_keys-openai.yaml",
    );

    await toggleOpenAIForm();
    await userEvent.type(screen.getByLabelText("Export from:"), "/");
    await userEvent.type(screen.getByLabelText("API key:"), "0p3n41-4p1-k3y");
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    await waitFor(() => expect(tauriInvokeMock).toHaveBeenCalledTimes(2));
    expect(getOpenAiStatus()).toBe("Active");
    // should be called twice -- once unsuccessfully to set the API key, once
    // successfully to get the new API key
    expect(tauriInvokeMock).toBeCalledTimes(2);
    expect(tauriInvokeMock).toHaveResolvedTimes(1);

    render(Snackbar, {});
    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent("Is a directory (os error 21)");
  });
});
