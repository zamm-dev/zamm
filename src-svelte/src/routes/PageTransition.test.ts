import { getTransitionType, TransitionType } from "./PageTransition.svelte";
import PageTransitionControl from "./PageTransitionControl.svelte";
import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { get } from "svelte/store";
import { vi } from "vitest";
import { firstPageLoad } from "$lib/firstPageLoad";

describe("Screen during transition", () => {
  it("should move towards the right if new route is subpath", () => {
    expect(getTransitionType("/parent", "/parent/child")).toEqual(
      TransitionType.Right,
    );
  });

  it("should move towards the left if new route is super-path", () => {
    expect(getTransitionType("/parent/child", "/parent")).toEqual(
      TransitionType.Left,
    );
  });

  it("should swap if both routes are different", () => {
    expect(getTransitionType("/some-page", "/other-page")).toEqual(
      TransitionType.Swap,
    );
  });

  it("should swap for root path", () => {
    expect(getTransitionType("/", "/some-page")).toEqual(TransitionType.Swap);
    expect(getTransitionType("/some-page", "/")).toEqual(TransitionType.Swap);
  });
});

describe("PageTransition", () => {
  let routeInput: HTMLElement;
  let navigateButton: HTMLElement;

  const navigateTo = async (url: string) => {
    await act(() => userEvent.type(routeInput, url));
    await act(() => userEvent.click(navigateButton));
  };

  beforeAll(() => {
    HTMLElement.prototype.animate = vi.fn().mockReturnValue({
      onfinish: null,
      cancel: vi.fn(),
    });
  });

  beforeEach(() => {
    render(PageTransitionControl, { currentRoute: "/" });
    routeInput = screen.getByLabelText("Route");
    navigateButton = screen.getByText("Navigate");
  });

  it("should set first page load on initial visit", async () => {
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should set first page load on visit to new page", async () => {
    await navigateTo("/settings");
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should unset first page load on visit to old page", async () => {
    await navigateTo("/settings");
    await navigateTo("/");
    expect(get(firstPageLoad)).toEqual(false);
  });
});
