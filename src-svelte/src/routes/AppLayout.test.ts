import { expect, test, vi, type Mock } from "vitest";
import { get } from "svelte/store";
import "@testing-library/jest-dom";
import { render, waitFor } from "@testing-library/svelte";
import AppLayout from "./AppLayout.svelte";
import {
  soundOn,
  volume,
  animationsOn,
  animationSpeed,
} from "$lib/preferences";
import { TauriInvokePlayback } from "$lib/sample-call-testing";
import { tickFor } from "$lib/test-helpers";

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);

describe("AppLayout", () => {
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

  test("will do nothing if no custom settings exist", async () => {
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-no-file.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await tickFor(3);
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("will set sound if sound preference overridden", async () => {
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-sound-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await waitFor(() => {
      expect(get(soundOn)).toBe(false);
    });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("will set volume if volume preference overridden", async () => {
    expect(get(volume)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-volume-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await waitFor(() => {
      expect(get(volume)).toBe(0.8);
    });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("will set animation if animation preference overridden", async () => {
    expect(get(animationsOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-animations-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await waitFor(() => {
      expect(get(animationsOn)).toBe(false);
    });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("will set animation speed if speed preference overridden", async () => {
    expect(get(animationSpeed)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    playback.addSamples(
      "../src-tauri/api/sample-calls/get_preferences-animation-speed-override.yaml",
    );

    render(AppLayout, { currentRoute: "/" });
    await waitFor(() => {
      expect(get(animationSpeed)).toBe(0.9);
    });
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });
});
