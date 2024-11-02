import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";
import DatabaseView, { resetDataType } from "./DatabaseView.svelte";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import {
  TauriInvokePlayback,
  stubGlobalInvoke,
} from "$lib/sample-call-testing";

class MockIntersectionObserver {
  constructor(callback: IntersectionObserverCallback) {
    setTimeout(() => {
      const entry: IntersectionObserverEntry = {
        isIntersecting: true,
        boundingClientRect: new DOMRectReadOnly(),
        intersectionRatio: 1,
        intersectionRect: new DOMRectReadOnly(),
        rootBounds: new DOMRectReadOnly(),
        target: document.createElement("div"),
        time: 0,
      };
      callback([entry], this as unknown as IntersectionObserver);
    }, 10);
  }

  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();
}

describe("Database View", () => {
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

    window.IntersectionObserver =
      MockIntersectionObserver as unknown as typeof IntersectionObserver;
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_api_calls-small.yaml",
    );
  });

  afterEach(() => {
    resetDataType();
  });

  test("renders LLM calls by default", async () => {
    render(DatabaseView, {
      dateTimeLocale: "en-GB",
      timeZone: "Asia/Phnom_Penh",
    });

    expect(screen.getByRole("heading")).toHaveTextContent("LLM API Calls");
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(1));
    await waitFor(() => {
      const linkToApiCall = screen.getByRole("link", {
        name: /yes, it works/i,
      });
      expect(linkToApiCall).toBeInTheDocument();
      expect(linkToApiCall).toHaveAttribute(
        "href",
        "/database/api-calls/d5ad1e49-f57f-4481-84fb-4d70ba8a7a74/",
      );
    });
  });

  test("can switch to rendering terminal sessions list", async () => {
    render(DatabaseView, {
      dateTimeLocale: "en-GB",
      timeZone: "Asia/Phnom_Penh",
    });
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(1));

    tauriInvokeMock.mockClear();
    playback.addSamples(
      "../src-tauri/api/sample-calls/get_terminal_sessions-small.yaml",
    );
    userEvent.selectOptions(
      screen.getByRole("combobox", { name: "Showing" }),
      "Terminal Sessions",
    );
    await waitFor(() => {
      expect(screen.getByRole("heading")).toHaveTextContent(
        "Terminal Sessions",
      );
    });
    await waitFor(() => expect(tauriInvokeMock).toHaveReturnedTimes(1));
    await waitFor(() => {
      const linkToTerminalSession = screen.getByRole("link", {
        name: /python api/i,
      });
      expect(linkToTerminalSession).toBeInTheDocument();
      expect(linkToTerminalSession).toHaveAttribute(
        "href",
        "/database/terminal-sessions/3717ed48-ab52-4654-9f33-de5797af5118/",
      );
    });
  });
});
