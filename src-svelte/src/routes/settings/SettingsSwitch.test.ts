import { assert, expect, test, vi } from "vitest";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import SettingsSwitch from "./SettingsSwitch.svelte";
import { stubGlobalInvoke } from "$lib/sample-call-testing";

const tauriInvokeMock = vi.fn();
stubGlobalInvoke(tauriInvokeMock);

describe("Settings switch", () => {
  test("can be toggled on from clicking the container", async () => {
    const { container } = render(SettingsSwitch, { label: "Test" });

    const switchContainer = container.querySelector(".settings-switch");
    assert(switchContainer);
    const onOffSwitch = screen.getByRole("switch");
    expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
    await act(() => userEvent.click(switchContainer));
    expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
  });
});
