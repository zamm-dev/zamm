import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";
import { render, screen, waitFor } from "@testing-library/svelte";
import TerminalSession from "./TerminalSession.svelte";
import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";
import { sidebar } from "../../SidebarUI.svelte";
import { pageTransition } from "../../PageTransition.svelte";
import { get } from "svelte/store";
import { replaceState } from "$app/navigation";

vi.mock("$app/navigation", () => {
  return { replaceState: vi.fn() };
});

describe("Terminal session", () => {
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

    (replaceState as Mock).mockClear();
    sidebar.set({ updateIndicator: vi.fn() });
    pageTransition.set({ addVisitedRoute: vi.fn() });

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

  test("can start and send input to command", async () => {
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
    expect(replaceState).toBeCalledWith(
      "/database/terminal-sessions/3717ed48-ab52-4654-9f33-de5797af5118/",
      undefined,
    );
    expect(get(sidebar)?.updateIndicator).toHaveBeenCalledWith(
      "/database/terminal-sessions/3717ed48-ab52-4654-9f33-de5797af5118/",
    );
    expect(get(pageTransition)?.addVisitedRoute).toHaveBeenCalledWith(
      "/database/terminal-sessions/3717ed48-ab52-4654-9f33-de5797af5118/",
    );

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
