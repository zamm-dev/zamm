import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";
import { render, screen, waitFor } from "@testing-library/svelte";
import UnloadedTerminalSession from "./UnloadedTerminalSession.svelte";
import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";

describe("Unloaded terminal session", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  beforeAll(() => {
    HTMLElement.prototype.animate = vi.fn().mockReturnValue({
      onfinish: null,
      cancel: vi.fn(),
    });
  });

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

  test("can resume active session", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_terminal_session-bash.yaml",
    );
    render(UnloadedTerminalSession, {
      id: "3717ed48-ab52-4654-9f33-de5797af5118",
    });

    // check that the page loads correctly
    await waitFor(() => {
      expect(tauriInvokeMock).toHaveReturnedTimes(1);
      expect(
        screen.getByText(
          new RegExp("The default interactive shell is now zsh"),
        ),
      ).toBeInTheDocument();
    });

    // check that we can still interact with the session
    playback.addSamples(
      "../src-tauri/api/sample-calls/send_command_input-bash-interleaved.yaml",
    );
    // this part differs from TerminalSession.test.ts
    const commandInput = screen.getByLabelText("Enter input for command");
    const sendButton = screen.getByRole("button", { name: "Send" });
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

  test("can load inactive session without allowing for further input", async () => {
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_terminal_session-bash-interleaved.yaml",
    );
    render(UnloadedTerminalSession, {
      id: "3717ed48-ab52-4654-9f33-de5797af5118",
    });

    // check that the page loads correctly
    await waitFor(() => {
      expect(tauriInvokeMock).toHaveReturnedTimes(1);
      const commandOutput = screen.getByText(
        new RegExp("The default interactive shell is now zsh"),
      );
      expect(commandOutput).toBeInTheDocument();
      expect(commandOutput).toHaveTextContent("stderr");
    });

    // check that we can no longer interact with the session
    const commandInput = screen.queryByLabelText("Enter input for command");
    // expect inpupt to not exist
    expect(commandInput).not.toBeInTheDocument();
    expect(
      screen.getByText("This terminal session is no longer active."),
    ).toBeInTheDocument();
  });
});
