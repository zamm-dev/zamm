import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";
import { render, screen, waitFor } from "@testing-library/svelte";
import TerminalSession from "./TerminalSession.svelte";
import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";

describe("Terminal session", () => {
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

  test("Terminal session", async () => {
    render(TerminalSession, {});
    const commandInput = screen.getByLabelText("Enter command to run");
    const sendButton = screen.getByRole("button", { name: "Send" });

    expect(tauriInvokeMock).not.toHaveBeenCalled();
    playback.addSamples("../src-tauri/api/sample-calls/run_command-bash.yaml");

    expect(commandInput).toHaveValue("");
    await userEvent.type(commandInput, "bash");
    await userEvent.click(sendButton);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    await waitFor(() => {
      expect(
        screen.getByText(
          new RegExp("The default interactive shell is now zsh"),
        ),
      ).toBeInTheDocument();
    });

    playback.addSamples(
      "../src-tauri/api/sample-calls/send_command_input-bash-interleaved.yaml",
    );
    expect(commandInput).toHaveValue("");
    await userEvent.type(
      commandInput,
      "python api/sample-terminal-sessions/interleaved.py",
    );
    await userEvent.click(sendButton);
    expect(tauriInvokeMock).toHaveReturnedTimes(2);
    await waitFor(() => {
      expect(screen.getByText(new RegExp("stderr"))).toBeInTheDocument();
    });
  });
});
