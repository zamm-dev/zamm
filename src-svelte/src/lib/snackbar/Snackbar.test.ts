import Snackbar, { snackbarError, clearAllMessages } from "./Snackbar.svelte";
import "@testing-library/jest-dom";
import { render, screen } from "@testing-library/svelte";
import { expect, vi } from "vitest";
import { tickFor } from "$lib/test-helpers";

describe("Snackbar", () => {
  beforeAll(() => {
    HTMLElement.prototype.animate = vi.fn().mockReturnValue({
      onfinish: null,
      cancel: vi.fn(),
    });
  });

  beforeEach(() => {
    clearAllMessages();

    vi.stubGlobal("requestAnimationFrame", (fn: FrameRequestCallback) => {
      return window.setTimeout(() => fn(Date.now()), 16);
    });
  });

  it("should not display any messages by default", () => {
    render(Snackbar, {});

    const alerts = screen.queryAllByRole("alert");
    expect(alerts).toHaveLength(0);
  });

  it("should display a message after an alert is triggered", async () => {
    const message = "This is a test message";
    render(Snackbar, {});
    snackbarError(message);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(message);
  });

  it("should be able to display multiple messages", async () => {
    const message1 = "This is a test message";
    const message2 = "This is another test message";
    render(Snackbar, {});
    snackbarError(message1);
    snackbarError(message2);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(2);
    expect(alerts[0]).toHaveTextContent(message1);
    expect(alerts[1]).toHaveTextContent(message2);
  });
});
