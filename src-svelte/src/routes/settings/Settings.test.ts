import { expect, test, vi, type Mock } from "vitest";
import { get } from "svelte/store";
import "@testing-library/jest-dom";
import { act, getByLabelText, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Settings from "./Settings.svelte";
import {
  soundOn,
  volume,
  transparencyOn,
  highDpiAdjust,
} from "$lib/preferences";
import {
  parseSampleCall,
  type ParsedCall,
  TauriInvokePlayback,
} from "$lib/sample-call-testing";

describe("Settings", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;

  let playSwitchSoundCall: ParsedCall;
  let setSoundOnCall: ParsedCall;
  let setSoundOffCall: ParsedCall;
  let setVolumePartialCall: ParsedCall;

  beforeAll(() => {
    global.ResizeObserver = class ResizeObserver {
      observe() {
        // do nothing
      }
      unobserve() {
        // do nothing
      }
      disconnect() {
        // do nothing
      }
    };

    playSwitchSoundCall = parseSampleCall(
      "../src-tauri/api/sample-calls/play_sound-switch.yaml",
    );
    setSoundOnCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-sound-on.yaml",
    );
    setSoundOffCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-sound-off.yaml",
    );
    setVolumePartialCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-volume-partial.yaml",
    );
  });

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );

    soundOn.set(true);
    volume.set(1);
    transparencyOn.set(false);
  });

  test("can toggle sound on and off while saving setting", async () => {
    render(Settings, {});
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const soundRegion = screen.getByRole("region", { name: "Sound" });
    const soundSwitch = getByLabelText(soundRegion, "Enabled");
    playback.addCalls(setSoundOffCall);
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(false);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(playback.unmatchedCalls.length).toBe(0);

    playback.addCalls(setSoundOnCall, playSwitchSoundCall);
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).toHaveReturnedTimes(3);
    expect(playback.unmatchedCalls.length).toBe(0);
  });

  test("can persist changes to volume slider", async () => {
    render(Settings, {});
    expect(get(volume)).toBe(1);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const soundRegion = screen.getByRole("region", { name: "Sound" });
    const volumeSlider = getByLabelText(soundRegion, "Volume");
    playback.addCalls(setVolumePartialCall);
    volumeSlider.focus();
    const user = userEvent.setup();
    await user.keyboard("[ArrowLeft]");
    expect(get(volume)).toBe(0.8);
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
    expect(playback.unmatchedCalls.length).toBe(0);
  });

  test("can toggle transparency on and off while saving setting", async () => {
    render(Settings, {});
    expect(get(transparencyOn)).toBe(false);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const otherVisualsRegion = screen.getByRole("region", {
      name: "Other visual effects",
    });
    const transparencySwitch = getByLabelText(
      otherVisualsRegion,
      "Transparency",
    );
    playback.addCalls(playSwitchSoundCall);
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_preferences-transparency-on.yaml",
    );
    await act(() => userEvent.click(transparencySwitch));
    expect(get(transparencyOn)).toBe(true);
    expect(tauriInvokeMock).toHaveReturnedTimes(2);
    expect(playback.unmatchedCalls.length).toBe(0);

    playback.addCalls(playSwitchSoundCall);
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_preferences-transparency-off.yaml",
    );
    await act(() => userEvent.click(transparencySwitch));
    expect(get(transparencyOn)).toBe(false);
    expect(tauriInvokeMock).toHaveReturnedTimes(4);
    expect(playback.unmatchedCalls.length).toBe(0);
  });

  test("can toggle high-dpi on and off while saving setting", async () => {
    render(Settings, {});
    expect(get(highDpiAdjust)).toBe(false);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const otherVisualsRegion = screen.getByRole("region", {
      name: "Other visual effects",
    });
    const highDpiSwitch = getByLabelText(otherVisualsRegion, "High DPI adjust");
    playback.addCalls(playSwitchSoundCall);
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_preferences-high-dpi-adjust-on.yaml",
    );
    await act(() => userEvent.click(highDpiSwitch));
    expect(get(highDpiAdjust)).toBe(true);
    expect(tauriInvokeMock).toHaveReturnedTimes(2);
    expect(playback.unmatchedCalls.length).toBe(0);

    playback.addCalls(playSwitchSoundCall);
    playback.addSamples(
      "../src-tauri/api/sample-calls/set_preferences-high-dpi-adjust-off.yaml",
    );
    await act(() => userEvent.click(highDpiSwitch));
    expect(get(highDpiAdjust)).toBe(false);
    expect(tauriInvokeMock).toHaveReturnedTimes(4);
    expect(playback.unmatchedCalls.length).toBe(0);
  });
});
