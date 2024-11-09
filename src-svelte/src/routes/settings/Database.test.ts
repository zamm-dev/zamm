import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen, waitFor } from "@testing-library/svelte";
import Snackbar, { clearAllMessages } from "$lib/snackbar/Snackbar.svelte";

import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";
import Database from "./Database.svelte";

describe("Individual API call", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    stubGlobalInvoke(tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );

    clearAllMessages();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  function mockFilePicker(action: string, path: string) {
    playback.addCalls({
      request: [`plugin:dialog|${action}`, { options: {} }],
      response: path,
      succeeded: true,
    });
  }

  async function checkForAlert(text: string) {
    render(Snackbar, {});
    await waitFor(() => {
      const alerts = screen.queryAllByRole("alertdialog");
      expect(alerts).toHaveLength(1);
      expect(alerts[0].textContent).toEqual(text);
    });
  }

  test("can export LLM calls", async () => {
    mockFilePicker("save", "test-folder/exported-db.yaml");
    playback.addSamples(
      "../src-tauri/api/sample-calls/export_db-conversations.yaml",
    );
    render(Database, {});

    const exportButton = screen.getByText("Export data");
    await act(() => userEvent.click(exportButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Exported 6 LLM calls");
  });

  test("can export API keys", async () => {
    mockFilePicker("save", "different.zamm.yaml");
    playback.addSamples("../src-tauri/api/sample-calls/export_db-api-key.yaml");
    render(Database, {});

    const exportButton = screen.getByText("Export data");
    await act(() => userEvent.click(exportButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Exported 1 API key");
  });

  test("can export terminal sessions", async () => {
    mockFilePicker("save", "exported-db.yaml");
    playback.addSamples(
      "../src-tauri/api/sample-calls/export_db-terminal-sessions.yaml",
    );
    render(Database, {});

    const exportButton = screen.getByText("Export data");
    await act(() => userEvent.click(exportButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Exported 1 terminal session");
  });

  test("can import LLM calls", async () => {
    mockFilePicker("open", "conflicting-db.yaml");
    playback.addSamples(
      "../src-tauri/api/sample-calls/import_db-conflicting-llm-call.yaml",
    );
    render(Database, {});

    const importButton = screen.getByText("Import data");
    await act(() => userEvent.click(importButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Imported 1 LLM call, ignored 1 LLM call");
  });

  test("can import API keys", async () => {
    mockFilePicker("open", "different.zamm.yaml");
    playback.addSamples("../src-tauri/api/sample-calls/import_db-api-key.yaml");
    render(Database, {});

    const importButton = screen.getByText("Import data");
    await act(() => userEvent.click(importButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Imported 1 API key");
  });

  test("can import terminal sessions", async () => {
    mockFilePicker("open", "exported-db.yaml");
    playback.addSamples(
      "../src-tauri/api/sample-calls/import_db-terminal-sessions.yaml",
    );
    render(Database, {});

    const importButton = screen.getByText("Import data");
    await act(() => userEvent.click(importButton));
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(2));

    await checkForAlert("Imported 1 terminal session");
  });
});
