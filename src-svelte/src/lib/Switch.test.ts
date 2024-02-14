import { expect, test, vi, type SpyInstance, type Mock } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Switch, { getClickDelayMs } from "./Switch.svelte";
import { soundOn, animationSpeed } from "$lib/preferences";
import { TauriInvokePlayback } from "$lib/sample-call-testing";

const tauriInvokeMock = vi.fn();
const recordSoundDelay = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
vi.stubGlobal("_testRecordSoundDelay", recordSoundDelay);

describe("Switch delay", () => {
  test("is 0 under regular animation speeds", () => {
    expect(getClickDelayMs(1)).toBe(0);
  });

  test("increases a little when animation is twice as slow", () => {
    expect(getClickDelayMs(0.5)).toBe(50);
  });

  test("increases a lot when animation is 10x as slow", () => {
    expect(getClickDelayMs(0.1)).toBe(450);
  });
});

describe("Switch", () => {
  let tauriInvokeMock: Mock;
  let playback: TauriInvokePlayback;
  let recordSoundDelaySpy: SpyInstance;

  beforeEach(() => {
    tauriInvokeMock = vi.fn();
    vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);
    playback = new TauriInvokePlayback();
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) =>
        playback.mockCall(...args),
    );
    playback.addSamples("../src-tauri/api/sample-calls/play_sound-switch.yaml");

    recordSoundDelaySpy = vi.spyOn(window, "_testRecordSoundDelay");
  });

  test("can be toggled on", async () => {
    render(Switch, {});

    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
    await act(() => userEvent.click(onOffSwitch));
    expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
  });

  test("can be toggled off", async () => {
    render(Switch, { toggledOn: true });

    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
    await act(() => userEvent.click(onOffSwitch));
    expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
  });

  test("can have multiple unique labels", async () => {
    render(Switch, { label: "One" });
    render(Switch, { label: "Two" });

    const switchOne = screen.getByLabelText("One");
    expect(switchOne).toHaveAttribute("aria-checked", "false");
    const switchTwo = screen.getByLabelText("Two");
    expect(switchTwo).toHaveAttribute("aria-checked", "false");

    await act(() => userEvent.click(switchOne));
    expect(switchOne).toHaveAttribute("aria-checked", "true");
    expect(switchTwo).toHaveAttribute("aria-checked", "false");

    await act(() => userEvent.click(switchTwo));
    expect(switchOne).toHaveAttribute("aria-checked", "true");
    expect(switchTwo).toHaveAttribute("aria-checked", "true");
  });

  test("plays clicking sound during toggle", async () => {
    render(Switch, {});
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(tauriInvokeMock).toHaveReturnedTimes(1);
  });

  test("does not play clicking sound when sound off", async () => {
    render(Switch, {});
    soundOn.update(() => false);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(tauriInvokeMock).not.toHaveBeenCalled();
  });

  test("does not usually delay click", async () => {
    render(Switch, {});
    expect(recordSoundDelaySpy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(recordSoundDelaySpy).toHaveBeenLastCalledWith(0);
  });

  test("delays click when animation speed is slow", async () => {
    animationSpeed.set(0.1);
    render(Switch, {});
    expect(recordSoundDelaySpy).not.toHaveBeenCalled();

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(recordSoundDelaySpy).toHaveBeenLastCalledWith(450);
  });

  test("calls onToggle when toggled", async () => {
    const onToggle = vi.fn();
    render(Switch, { onToggle });

    const onOffSwitch = screen.getByRole("switch");
    await act(() => userEvent.click(onOffSwitch));
    expect(onToggle).toHaveReturnedTimes(1);
  });
});
