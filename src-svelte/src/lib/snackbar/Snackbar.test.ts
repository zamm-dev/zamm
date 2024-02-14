import Snackbar, { snackbarError, clearAllMessages } from "./Snackbar.svelte";
import "@testing-library/jest-dom";
import { within, waitFor } from "@testing-library/dom";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { expect, vi } from "vitest";
import { tickFor } from "$lib/test-helpers";

describe("Snackbar", () => {
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

  it("should hide a message if the dismiss button is clicked", async () => {
    const message = "This is a test message";
    render(Snackbar, {});
    snackbarError(message);
    await tickFor(3);

    const alerts = screen.queryAllByRole("alertdialog");
    expect(alerts).toHaveLength(1);
    expect(alerts[0]).toHaveTextContent(message);

    const dismissButton = within(alerts[0]).getByRole("button", {
      name: "Dismiss",
    });
    await userEvent.click(dismissButton);
    await waitFor(() => expect(alerts[0]).not.toBeInTheDocument());
    const alertsAfterDismiss = screen.queryAllByRole("alertdialog");
    expect(alertsAfterDismiss).toHaveLength(0);
  });
});
