import { expect, test, vi, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import SidebarUI from "./SidebarUI.svelte";
import { soundOn, volume, animationSpeed } from "$lib/preferences";

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);

describe("Sidebar", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;
  let homeLink: HTMLElement;
  let settingsLink: HTMLElement;

  beforeAll(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
    playback.addSamples("../src-tauri/api/sample-calls/play_sound-whoosh.yaml");
  });

  beforeEach(() => {
    render(SidebarUI, {
      currentRoute: "/",
      dummyLinks: true,
    });
    homeLink = screen.getByTitle("Dashboard");
    settingsLink = screen.getByTitle("Settings");
    expect(homeLink).toHaveAttribute("aria-current", "page");
    expect(settingsLink).not.toHaveAttribute("aria-current", "page");
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  test("can change page path", async () => {
    await act(() => userEvent.click(settingsLink));
    expect(homeLink).not.toHaveAttribute("aria-current", "page");
    expect(settingsLink).toHaveAttribute("aria-current", "page");
  });

  test("plays whoosh sound with right speed and volume", async () => {
    // volume is at 0.125 so that when it's boosted 4x to compensate for the 4x
    // reduction in playback speed, the net volume will be at 0.5 as specified in the
    // sample file
    volume.update(() => 0.125);
    animationSpeed.update(() => 0.25);
    await act(() => userEvent.click(settingsLink));
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("does not play whoosh sound when sound off", async () => {
    soundOn.update(() => false);

    await act(() => userEvent.click(settingsLink));
    expect(homeLink).not.toHaveAttribute("aria-current", "page");
    expect(settingsLink).toHaveAttribute("aria-current", "page");
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  test("does not play whoosh sound when path unchanged", async () => {
    await act(() => userEvent.click(homeLink));
    expect(homeLink).toHaveAttribute("aria-current", "page");
    expect(settingsLink).not.toHaveAttribute("aria-current", "page");
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });
});
