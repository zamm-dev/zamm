import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { render, screen } from "@testing-library/svelte";
import Metadata from "./Metadata.svelte";
import { within, waitFor } from "@testing-library/dom";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { systemInfo } from "$lib/system-info";
import { get } from "svelte/store";

describe("Metadata", () => {
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

  test("loading by default", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_system_info-linux.yaml",
    );

    render(Metadata, {});

    const status = screen.getByRole("status");
    expect(status).toHaveTextContent(/^...loading$/);
  });

  test("linux system info returned", async () => {
    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_system_info-linux.yaml",
    );

    render(Metadata, {});
    await waitFor(() => {
      const shellRow = screen.getByRole("row", { name: /Shell/ });
      const shellValueCell = within(shellRow).getAllByRole("cell")[1];
      expect(shellValueCell).toHaveTextContent("Zsh");
      expect(get(systemInfo)?.shell_init_file).toEqual("/root/.zshrc");
    });

    expect(tauriInvokeMock).toHaveReturnedTimes(1);

    const versionRow = screen.getByRole("row", { name: /Version/ });
    const versionValueCell = within(versionRow).getAllByRole("cell")[1];
    expect(versionValueCell).toHaveTextContent("0.0.0");

    const osRow = screen.getByRole("row", { name: /OS/ });
    const osValueCell = within(osRow).getAllByRole("cell")[1];
    expect(osValueCell).toHaveTextContent("Linux");
  });

  test("API key error", async () => {
    const spy = vi.spyOn(window, "__TAURI_INVOKE__");
    expect(spy).not.toHaveBeenCalled();
    tauriInvokeMock.mockRejectedValueOnce("testing");

    render(Metadata, {});
    expect(spy).toHaveReturnedTimes(1);

    await waitFor(() => {
      const status = screen.getByRole("status");
      expect(status).toHaveTextContent(/^error: testing$/);
    });
  });
});
