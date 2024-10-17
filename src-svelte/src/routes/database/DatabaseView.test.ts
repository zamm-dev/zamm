import { expect, test, vi } from "vitest";
import "@testing-library/jest-dom";
import DatabaseView, { resetDataType } from "./DatabaseView.svelte";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";

describe("Database View", () => {
  beforeEach(() => {
    window.IntersectionObserver = vi.fn(() => {
      return {
        observe: vi.fn(),
        unobserve: vi.fn(),
        disconnect: vi.fn(),
      };
    }) as unknown as typeof IntersectionObserver;
  });

  afterEach(() => {
    resetDataType();
  });

  test("renders LLM calls by default", async () => {
    render(DatabaseView, {
      dateTimeLocale: "en-GB",
      timeZone: "Asia/Phnom_Penh",
    });

    const title = screen.getByRole("heading");
    expect(title).toHaveTextContent("LLM API Calls");
    userEvent.selectOptions(
      screen.getByRole("combobox", { name: "Showing" }),
      "Terminal Sessions",
    );
    await waitFor(() => {
      expect(title).toHaveTextContent("Terminal Sessions");
    });
  });
});
